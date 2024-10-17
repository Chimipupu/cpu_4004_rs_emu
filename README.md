# Rust Intel 4004エミュレータ(開発中)

Rustで実装した世界初(1971年)のCPU `Intel 4004`のエミュレータ🥳
※絶賛開発中です！

# Intel 4004

![](https://storage.googleapis.com/zenn-user-upload/0445e8c44e66-20241017.png)

## リファレンス

📍Intel 4004は下記文献をリファレンスにしました🥳

📚[Intel 4004 データシート]
   - https://deramp.com/downloads/mfe_archive/050-Component%20Specifications/Intel/Microprocessors%20and%20Support/4004%20Family/4004Data.pdf

📚[MCS4 Assembly Language Programming Manual]
   - https://codeabbey.github.io/heavy-data-1/msc-4-asm-manual-1973.pdf

📚[Detailed Instruction Repertoire of the MCS-4]
  - http://e4004.szyc.org/iset.html

## 仕様

- バス幅: 4bit
- クロック: 740kHz
- 命令セット: 46
- パッケージ: 16Pin DIP
- ACC(アキュムレータ): 4bit
- フラグ: キャリーフラグのみ
- インデックスレジスタ
  - 4bit x16の個別
    - R0〜R15
  - 8bit x8のペア
    - R0P ～R7P
- SP: 12bit x3レベル(ネスト3段まで)
- PC: 12bit
- ROM: 最大 32768 bit (4096 Byte)
- RAM:
  -  プログラムRAM: 最大 32768 bit (4096 Byte)
  -  データRAM: 最大 5120 bit  (640 Byte)

## 命令セット
[Rustでの実装状況]
- ✅ ... 実装済み (5/46)
- ❌ ... TBD

| No. | Category                                | Instruction | Description (英語 / 日本語)                                   |
|-----|-----------------------------------------|-------------|----------------------------------------------------------|
| 1   | INDEX REGISTER INSTRUCTIONS             | INC         | Increment index register / インデックスレジスタをインクリメント |
| 2   | INDEX REGISTER INSTRUCTIONS             | FIN         | Finish operation / 操作を終了する                            |
| 3   | INDEX REGISTER TO ACCUMULATOR INSTRUCTIONS | ADD         | Add index register to accumulator / インデックスレジスタをアキュムレータに加算 |
| 4   | INDEX REGISTER TO ACCUMULATOR INSTRUCTIONS | SUB         | Subtract index register from accumulator / アキュムレータからインデックスレジスタを減算 |
| 5   | INDEX REGISTER TO ACCUMULATOR INSTRUCTIONS | ✅LD          | Load index register to accumulator / インデックスレジスタをアキュムレータにロード |
| 6   | INDEX REGISTER TO ACCUMULATOR INSTRUCTIONS | ✅XCH         | Exchange index register with accumulator / インデックスレジスタとアキュムレータを交換 |
| 7   | ACCUMULATOR INSTRUCTIONS                | CLB         | Clear accumulator / アキュムレータをクリア                     |
| 8   | ACCUMULATOR INSTRUCTIONS                | CLC         | Clear carry / キャリーをクリア                                 |
| 9   | ACCUMULATOR INSTRUCTIONS                | IAC         | Increment accumulator / アキュムレータをインクリメント          |
| 10  | ACCUMULATOR INSTRUCTIONS                | CMC         | Complement carry / キャリーを補完                              |
| 11  | ACCUMULATOR INSTRUCTIONS                | CMA         | Complement accumulator / アキュムレータを補完                   |
| 12  | ACCUMULATOR INSTRUCTIONS                | RAL         | Rotate accumulator left / アキュムレータを左にローテート        |
| 13  | ACCUMULATOR INSTRUCTIONS                | RAR         | Rotate accumulator right / アキュムレータを右にローテート       |
| 14  | ACCUMULATOR INSTRUCTIONS                | TCC         | Transfer carry to accumulator / キャリーをアキュムレータに転送     |
| 15  | ACCUMULATOR INSTRUCTIONS                | DAC         | Decrement accumulator / アキュムレータをデクリメント             |
| 16  | ACCUMULATOR INSTRUCTIONS                | TCS         | Transfer accumulator to carry / アキュムレータをキャリーに転送   |
| 17  | ACCUMULATOR INSTRUCTIONS                | STC         | Set carry / キャリーをセット                                   |
| 18  | ACCUMULATOR INSTRUCTIONS                | DAA         | Decimal adjust accumulator / アキュムレータを十進法で調整       |
| 19  | ACCUMULATOR INSTRUCTIONS                | KBP         | Keyboard process / キーボードプロセス                        |
| 20  | IMMEDIATE INSTRUCTIONS                  | ✅FIM         | Fetch immediate / 即時取得                                    |
| 21  | IMMEDIATE INSTRUCTIONS                  | ✅LDM         | Load accumulator immediate / アキュムレータを即時にロード        |
| 22  | TRANSFER OF CONTROL INSTRUCTIONS        | JUN         | Jump unconditionally / 条件なしジャンプ                         |
| 23  | TRANSFER OF CONTROL INSTRUCTIONS        | JIN         | Jump indirect / 間接ジャンプ                                   |
| 24  | TRANSFER OF CONTROL INSTRUCTIONS        | JCN         | Jump on condition / 条件付きジャンプ                           |
| 25  | TRANSFER OF CONTROL INSTRUCTIONS        | ISZ         | Increment and skip if zero / ゼロの場合はインクリメントしてスキップ |
| 26  | SUBROUTINE LINKAGE INSTRUCTIONS         | JMS         | Jump to subroutine / サブルーチンへジャンプ                    |
| 27  | SUBROUTINE LINKAGE INSTRUCTIONS         | BBL         | Branch back and load / ブランチバックしてロード                  |
| 28  | NOP                                     | ✅NOP         | No operation / ノーオペレーション                               |
| 29  | MEMORY SELECTION INSTRUCTIONS            | DCL         | Designate command line / コマンドラインを指定                   |
| 30  | MEMORY SELECTION INSTRUCTIONS            | SRC         | Send register control / レジスタコントロールを送信               |
| 31  | I/O AND RAM INSTRUCTIONS                | WRM         | Write data RAM character / データRAM文字を書き込む              |
| 32  | I/O AND RAM INSTRUCTIONS                | WMP         | Write RAM port / RAMポートに書き込む                          |
| 33  | I/O AND RAM INSTRUCTIONS                | WRR         | Write ROM port / ROMポートに書き込む                          |
| 34  | I/O AND RAM INSTRUCTIONS                | WPM         | Write program RAM / プログラムRAMに書き込む                    |
| 35～38  | I/O AND RAM INSTRUCTIONS                | WR[0:3]         | Write data RAM status character / データRAMステータス文字を書き込む |
| 39  | I/O AND RAM INSTRUCTIONS                | RDM         | Read data RAM data character / データRAMデータ文字を読み込む     |
| 40  | I/O AND RAM INSTRUCTIONS                | RDR         | Read ROM port / ROMポートを読み込む                           |
| 41～44  | I/O AND RAM INSTRUCTIONS                | RD[0:3]         | Read data RAM status character / データRAMステータス文字を読み込む |
| 45  | I/O AND RAM INSTRUCTIONS                | ADM         | Add data RAM to accumulator with carry / キャリーを伴ってデータRAMをアキュムレータに加算 |
| 46  | I/O AND RAM INSTRUCTIONS                | SBM         | Subtract data RAM from memory with borrow / メモリからデータRAMを減算 |

# 参考文献
🎦動画[NHKスペシャル　電子立国　日本の自叙伝　第5回　8ミリ角のコンピュータ]
- https://www.nhk-ondemand.jp/goods/G2011034631SA000/index.html

📚嶋正利さん著書「マイクロコンピュータの誕生:わが青春の4004」
- https://ci.nii.ac.jp/ncid/BN01286436#anc-library

📚嶋正利さん[マイクロプロセッサ 4004の開発]
- https://www.shmj.or.jp/dev_story/pdf/develop43.pdf

📚[Intel 4004 データシート]
- https://deramp.com/downloads/mfe_archive/050-Component%20Specifications/Intel/Microprocessors%20and%20Support/4004%20Family/4004Data.pdf

📚MCS-4アセンブラのマニュアル[MCS4 Assembly Language Programming Manual]
- https://codeabbey.github.io/heavy-data-1/msc-4-asm-manual-1973.pdf

📚[Detailed Instruction Repertoire of the MCS-4]
- http://e4004.szyc.org/iset.html

📚[Intel公式4004関連]
- https://www.intel.co.jp/content/www/jp/ja/newsroom/news/intel-marks-50th-anniversary-4004.html
- https://www.intel.com/content/www/us/en/newsroom/resources/intel-4004.html
- https://www.intel.com/content/www/us/en/history/museum-story-of-intel-4004.html

📚日経[電卓向けの性格を色濃く残す、4004のアーキテクチャー]
- https://xtech.nikkei.com/dm/article/FEATURE/20141212/394135/
