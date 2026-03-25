// #[cfg(target_arch = "x86_64")]
// use std::arch::x86_64::*;
//
// // SIMD命令を使うため、ターゲット機能のチェックかunsafeブロックが必要です
// // 実際には #[target_feature(enable = "sse4.2")] をつけた関数内で呼ぶのが一般的です
// #[target_feature(enable = "sse4.2")]
// unsafe fn find_delimiter_index() -> i32 {
// 	// xmm1: パターン (",\r\n")
// 	// 実際には16バイトのアライメントやパディングを考慮する必要がありますが、
// 	// loadu (unaligned load) を使うのでu8スライスから直接読み込めます。
// 	let pattern = b",\r\n";
// 	let input = b"12345678901234,";
//
// 	// データのロード (__m128i型へ)
// 	// _mm_loadu_si128 はアライメントされていないメモリからのロードを許容します
// 	// 実際の運用ではバッファの終端処理（オーバーリード）に注意が必要です
// 	let a = _mm_loadu_si128(pattern.as_ptr() as *const __m128i);
// 	let b = _mm_loadu_si128(input.as_ptr() as *const __m128i);
//
// 	// 長さの指定 (Explicit Length)
// 	let la = pattern.len() as i32; // 3
// 	let lb = input.len() as i32; // 5
//
// 	// imm8 (制御バイト) の構築
// 	// bit 1:0 (Format) : 00 (Unsigned Byte)
// 	// bit 3:2 (Mode)   : 00 (Equal Any) -> strcspn相当
// 	// bit 5:4 (Polarity): 00 (Positive)
// 	// bit 6   (Index)  : 0  (LSB: 最初にマッチしたインデックス)
// 	const CONTROL: i32 = 0b00_00_00_00; // 0x00
//
// 	// 命令実行: pcmpestri xmm1, xmm2, imm8
// 	_mm_cmpestri(a, la, b, lb, CONTROL)
// }
//
// #[target_feature(enable = "sse4.2")]
// unsafe fn analyze_chunk_mask() -> i32 {
// 	// 1. 検索パターン (",\r\n")
// 	// ※実際には外で定義して使い回すのが良いです
// 	let pattern_bytes = b"\",\r\n";
// 	let input_chunk = b",1245678901234,";
// 	// 比較対象（入力データの16バイト）
// 	// loadu相当の処理（input_chunkは16バイト以上ある前提）
// 	let a = _mm_loadu_si128(pattern_bytes.as_ptr() as *const __m128i);
// 	let b = _mm_loadu_si128(input_chunk.as_ptr() as *const __m128i);
//
// 	let la = pattern_bytes.len() as i32; // 4
// 	let lb = 16; // 入力はフルサイズ16バイトと仮定
//
// 	// 2. imm8 (制御バイト) の設定
// 	// bit 1:0 (Format)   : 00 (Unsigned Byte)
// 	// bit 3:2 (Mode)     : 00 (Equal Any) -> いずれかの文字に一致
// 	// bit 5:4 (Polarity) : 00 (Positive)  -> 一致したらビットを立てる
// 	// bit 6   (Output)   : 0  (Bit Mask)  -> 結果をビット列としてXMM0の下位に詰める
// 	//                      ※ここを1にするとByte Mask（00 or FFの配列）になります
// 	const CONTROL: i32 = 0b00_00_00_00;
//
// 	// 3. 命令実行: pcmpestrm
// 	// 戻り値は __m128i 型 (XMM0レジスタの内容)
// 	let mask_xmm = _mm_cmpestrm(a, la, b, lb, CONTROL);
//
// 	// 4. SIMDレジスタから汎用レジスタへ取り出し
// 	// XMM0の第0要素(低位32ビット)をi32として取り出します
// 	let mask = _mm_cvtsi128_si32(mask_xmm);
//
// 	mask
// }
//
// fn main() {
// 	if is_x86_feature_detected!("sse4.2") {
// 		let idx = unsafe { find_delimiter_index() };
// 		println!("Index found: {}", idx); // 出力: 1
//
// 		let idx = unsafe { analyze_chunk_mask() };
// 		println!("Mask:{:b}", idx as u16);
// 	} else {
// 		println!("SSE4.2 not supported on this CPU.");
// 	}
// }

fn main() {}
