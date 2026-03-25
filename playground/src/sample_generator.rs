use rand::RngCore;
use rand::SeedableRng;
use rand::{Rng, random_bool};
use rand_distr::Distribution;
use rand_pcg::Pcg64Mcg;

const DOUBLE_QUOTE: char = '"';
const ESCAPES: [char; 3] = ['\n', '\r', '\t'];

const CANDIDATE: [char; 270] = [
	' ', '!', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', '0', '1', '2', '3',
	'4', '5', '6', '7', '8', '9', ':', ';', '<', '=', '>', '?', '@', 'A', 'B', 'C', 'D', 'E', 'F',
	'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y',
	'Z', '[', '\\', ']', '^', '_', '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
	'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '{', '|', '}', '~', 'ぁ',
	'あ', 'ぃ', 'い', 'ぅ', 'う', 'ぇ', 'え', 'ぉ', 'お', 'か', 'が', 'き', 'ぎ', 'く', 'ぐ', 'け',
	'げ', 'こ', 'ご', 'さ', 'ざ', 'し', 'じ', 'す', 'ず', 'せ', 'ぜ', 'そ', 'ぞ', 'た', 'だ', 'ち',
	'ぢ', 'っ', 'つ', 'づ', 'て', 'で', 'と', 'ど', 'な', 'に', 'ぬ', 'ね', 'の', 'は', 'ば', 'ぱ',
	'ひ', 'び', 'ぴ', 'ふ', 'ぶ', 'ぷ', 'へ', 'べ', 'ぺ', 'ほ', 'ぼ', 'ぽ', 'ま', 'み', 'む', 'め',
	'も', 'ゃ', 'や', 'ゅ', 'ゆ', 'ょ', 'よ', 'ら', 'り', 'る', 'れ', 'ろ', 'ゎ', 'わ', 'ゐ', 'ゑ',
	'を', 'ん', 'ゔ', 'ゕ', 'ゖ', 'ァ', 'ア', 'ィ', 'イ', 'ゥ', 'ウ', 'ェ', 'エ', 'ォ', 'オ', 'カ',
	'ガ', 'キ', 'ギ', 'ク', 'グ', 'ケ', 'ゲ', 'コ', 'ゴ', 'サ', 'ザ', 'シ', 'ジ', 'ス', 'ズ', 'セ',
	'ゼ', 'ソ', 'ゾ', 'タ', 'ダ', 'チ', 'ヂ', 'ッ', 'ツ', 'ヅ', 'テ', 'デ', 'ト', 'ド', 'ナ', 'ニ',
	'ヌ', 'ネ', 'ノ', 'ハ', 'バ', 'パ', 'ヒ', 'ビ', 'ピ', 'フ', 'ブ', 'プ', 'ヘ', 'ベ', 'ペ', 'ホ',
	'ボ', 'ポ', 'マ', 'ミ', 'ム', 'メ', 'モ', 'ャ', 'ヤ', 'ュ', 'ユ', 'ョ', 'ヨ', 'ラ', 'リ', 'ル',
	'レ', 'ロ', 'ヮ', 'ワ', 'ヰ', 'ヱ', 'ヲ', 'ン', 'ヴ', 'ヵ', 'ヶ', 'ヷ', 'ヸ', 'ヹ', 'ヺ',
];

pub fn get_char<T: RngCore>(rng: &mut T) -> char {
	CANDIDATE[rng.random_range(0..270)]
}

pub fn gen_sample(
	seed: u64,
	total_length: usize,
	len_sig: f64,
	len_dev: f64,
	double_quote: f64,
	escape: f64,
) -> Vec<String> {
	let mut rng = Pcg64Mcg::seed_from_u64(seed);
	let dist = rand_distr::Normal::new(len_sig, len_dev).unwrap();
	let mut vec = Vec::<String>::new();

	let mut cnt = 0usize;

	loop {
		let length = loop {
			let len = dist.sample(&mut rng);
			if len > 1.0 {
				let len = len as usize;
				break len;
			}
		};

		let mut buffer = String::with_capacity(length);
		let dq = rng.random_bool(double_quote);

		if dq {
			buffer.push(DOUBLE_QUOTE);
		}

		for _ in 0..length {
			if rng.random_bool(escape) {
				if rng.random_bool(0.5) {
					buffer.push(ESCAPES[rng.random_range(0..3)]);
				} else {
					buffer.push(DOUBLE_QUOTE);
					buffer.push(DOUBLE_QUOTE);
				}
			} else {
				buffer.push(get_char(&mut rng));
			}
		}

		if dq {
			buffer.push(DOUBLE_QUOTE);
		}

		cnt += buffer.len();
		vec.push(buffer);
		if cnt >= total_length {
			break;
		}
	}

	vec
}
