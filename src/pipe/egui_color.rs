// src/pipe/egui_color.rs

use egui::Color32;

/// Rozszerzenie Color32 o przestrzenie barw OKLab i OKLCH.
pub trait Color32OklabExt {
	fn from_oklab(l: f32, a: f32, b: f32) -> Self;
	fn from_oklch(l: f32, c: f32, h: f32) -> Self;
}

impl Color32OklabExt for Color32 {
	fn from_oklab(l: f32, a: f32, b: f32) -> Self {
		oklab(l, a, b)
	}

	fn from_oklch(l: f32, c: f32, h: f32) -> Self {
		oklch(l, c, h)
	}
}

/// Helper: Konwersja Oklab do liniowego sRGB (zwraca r, g, b bez obcinania)
const fn oklab_to_linear_srgb(l: f32, a: f32, b: f32) -> (f32, f32, f32) {
	let l_ = l + 0.3963377774 * a + 0.2158037573 * b;
	let m_ = l - 0.1055613458 * a - 0.0638541728 * b;
	let s_ = l - 0.0894841775 * a - 1.2914855480 * b;

	let ll = l_ * l_ * l_;
	let mm = m_ * m_ * m_;
	let ss = s_ * s_ * s_;

	let r = 4.0767416621 * ll - 3.3077115913 * mm + 0.2309699292 * ss;
	let g = -1.2684380046 * ll + 2.6097574011 * mm - 0.3413193965 * ss;
	let b = -0.0041960863 * ll - 0.7034186147 * mm + 1.7076147010 * ss;

	(r, g, b)
}

/// Helper: Sprawdza czy składowe RGB mieszczą się w gamucie sRGB (0.0 - 1.0)
const fn is_in_srgb_gamut(r: f32, g: f32, b: f32) -> bool {
	r >= 0.0 && r <= 1.0 && g >= 0.0 && g <= 1.0 && b >= 0.0 && b <= 1.0
}

/// Konwersja OKLab do sRGB — dostępna w czasie kompilacji.
pub const fn oklab(l: f32, a: f32, b: f32) -> Color32 {
	if l <= 0.0 {
		return Color32::BLACK;
	}
	let (r, g, b) = oklab_to_linear_srgb(l, a, b);
	Color32::from_rgb(channel(r), channel(g), channel(b))
}

/// Konwersja OKLCH (L, C, h w stopniach) do sRGB — const.
pub const fn oklch(l: f32, c: f32, h: f32) -> Color32 {
	if l <= 0.0 {
		return Color32::BLACK;
	}
	let l_clamped = if l < 0.0 {
		0.0
	} else if l > 1.0 {
		1.0
	} else {
		l
	};
	if c <= 0.0 {
		return oklab(l, 0.0, 0.0);
	}
	let (sin_h, cos_h) = sin_cos_degrees(h);

	let (mut r, mut g, mut b) = oklab_to_linear_srgb(l_clamped, c * cos_h, c * sin_h);

	if !is_in_srgb_gamut(r, g, b) {
		let mut min_c = 0.0;
		let mut max_c = c;
		let mut iters = 0;

		// BAZA: Rozpoczynamy od gwarantowanego koloru w gamucie (szarość dla danego L)
		let (mut best_r, mut best_g, mut best_b) = oklab_to_linear_srgb(l_clamped, 0.0, 0.0);

		while iters < 16 {
			let mid_c = min_c + (max_c - min_c) / 2.0;
			let (tr, tg, tb) = oklab_to_linear_srgb(l_clamped, mid_c * cos_h, mid_c * sin_h);

			if is_in_srgb_gamut(tr, tg, tb) {
				best_r = tr;
				best_g = tg;
				best_b = tb;
				min_c = mid_c;
			} else {
				max_c = mid_c;
			}
			iters += 1;
		}

		r = best_r;
		g = best_g;
		b = best_b;
	}

	Color32::from_rgb(channel(r), channel(g), channel(b))
}

const fn channel(linear: f32) -> u8 {
	let x = if linear < 0.0 {
		0.0
	} else if linear > 1.0 {
		1.0
	} else {
		linear
	};

	let srgb = if x <= 0.0031308 {
		12.92 * x
	} else {
		1.055 * pow_five_twelths(x) - 0.055
	};

	(srgb * 255.0 + 0.5) as u8
}

/// Oblicza x^(5/12) metodą Newtona:
/// Rozwiązujemy równanie y^12 = x, a następnie zwracamy y^5.
const fn pow_five_twelths(x: f32) -> f32 {
	let mut y = 1.0_f32;
	let mut i = 0;

	// Znajdujemy y = x^(1/12)
	while i < 16 {
		let y2 = y * y;
		let y4 = y2 * y2;
		let y8 = y4 * y4;
		let y11 = y8 * y2 * y;
		let y12 = y11 * y;

		y = y - (y12 - x) / (12.0 * y11);
		i += 1;
	}

	// Podnosimy wynik do potęgi 5, otrzymując x^(5/12)
	let y2 = y * y;
	let y4 = y2 * y2;
	y4 * y
}

/// Sinus i cosinus kąta w stopniach.
const fn sin_cos_degrees(degrees: f32) -> (f32, f32) {
	let mut angle = degrees;

	angle = angle % 360.0;
	if angle < 0.0 {
		angle += 360.0;
	}
	if angle > 180.0 {
		angle -= 360.0;
	}

	let mut x = angle * 0.017453292519943295;
	let mut cos_sign = 1.0_f32;

	if x > 1.5707963267948966 {
		x = 3.141592653589793 - x;
		cos_sign = -1.0;
	} else if x < -1.5707963267948966 {
		x = -3.141592653589793 - x;
		cos_sign = -1.0;
	}

	let x2 = x * x;

	let sin = x
		* (1.0
			+ x2 * (-1.0 / 6.0
				+ x2 * (1.0 / 120.0
					+ x2 * (-1.0 / 5040.0
						+ x2 * (1.0 / 362880.0
							+ x2 * (-1.0 / 39916800.0 + x2 * (1.0 / 6227020800.0)))))));
	let cos = 1.0
		+ x2 * (-1.0 / 2.0
			+ x2 * (1.0 / 24.0
				+ x2 * (-1.0 / 720.0
					+ x2 * (1.0 / 40320.0
						+ x2 * (-1.0 / 3628800.0
							+ x2 * (1.0 / 479001600.0 + x2 * (-1.0 / 87178291200.0)))))));

	(sin, cos * cos_sign)
}

#[cfg(test)]
mod tests {
	use super::*;
	use palette::{FromColor, Oklab, Oklch, Srgb};

	// Helper: tolerancja dla różnic precyzji w obliczeniach const vs std
	const TOLERANCE: u8 = 1;

	fn assert_color_eq(actual: Color32, expected: Srgb<u8>) {
		assert!(
			actual.r().abs_diff(expected.red) <= TOLERANCE
				&& actual.g().abs_diff(expected.green) <= TOLERANCE
				&& actual.b().abs_diff(expected.blue) <= TOLERANCE,
			"Kolory się różnią! Wynik const: {:?}, Oczekiwano (palette): {:?}",
			actual,
			expected
		);
	}

	#[test]
	fn test_oklab_matches_palette() {
		let cases = [
			(0.5, 0.1, -0.1),
			(0.8, -0.05, 0.05),
			(0.2, 0.0, 0.0), // Szarość
		];

		for (l, a, b) in cases {
			let actual = oklab(l, a, b);
			let palette_lab = Oklab::new(l, a, b);
			let palette_srgb: Srgb<u8> = Srgb::from_color(palette_lab).into_format();

			assert_color_eq(actual, palette_srgb);
		}
	}

	#[test]
	fn test_oklch_matches_palette() {
		let cases = [(0.5, 0.1, 45.0), (0.8, 0.05, 180.0), (0.3, 0.15, 300.0)];

		for (l, c, h) in cases {
			let actual = oklch(l, c, h);
			let palette_lch = Oklch::new(l, c, h);
			let palette_srgb: Srgb<u8> = Srgb::from_color(palette_lch).into_format();

			assert_color_eq(actual, palette_srgb);
		}
	}

	#[test]
	fn test_edge_cases() {
		// L = 0 (Czerń)
		assert_eq!(oklch(0.0, 0.5, 100.0), Color32::from_rgb(0, 0, 0));
		assert_eq!(oklab(0.0, 0.5, 0.5), Color32::from_rgb(0, 0, 0));

		// L = 1 (Biel) - zakłada zerową chromę po clippingu
		assert_eq!(oklch(1.0, 0.0, 0.0), Color32::from_rgb(255, 255, 255));

		// Ujemna chroma (powinna być traktowana jak 0)
		let actual_neg_c = oklch(0.5, -0.1, 45.0);
		let actual_zero_c = oklch(0.5, 0.0, 45.0);
		assert_eq!(actual_neg_c, actual_zero_c);

		// Kąty ujemne i > 360
		let base = oklch(0.5, 0.1, 45.0);
		assert_eq!(oklch(0.5, 0.1, 405.0), base, "Kąt > 360 zawiódł");
		assert_eq!(oklch(0.5, 0.1, -315.0), base, "Kąt ujemny zawiódł");
	}

	#[test]
	fn test_gamut_mapping_out_of_bounds() {
		// Bardzo nasycony niebieski z dużym L - niemożliwy w sRGB
		let l = 0.9;
		let c = 0.4;
		let h = 265.0; // Odcień błękitu/fioletu

		let result = oklch(l, c, h);

		// Zamiast zwracać "przepalony" kolor (np. 255, 255, 255 lub coś wpadającego w magenta),
		// Gamut mapping powinien obniżyć chromę, by zwrócić jasny błękit mieszczący się w gamucie.
		// Oczekujemy wartości, które nie zostały chamsko ucięte do 0/255.
		assert!(
			result.r() > 0 && result.r() < 255,
			"R powinno zostać sprowadzone do wnętrza gamutu"
		);
		assert!(
			result.g() > 0 && result.g() < 255,
			"G powinno zostać sprowadzone do wnętrza gamutu"
		);
		assert!(result.b() == 255, "B zazwyczaj ląduje na krawędzi gamutu");
	}
}
