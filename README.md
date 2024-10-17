# Rust Intel 4004 Emulator

# Intel 4004

![](https://storage.googleapis.com/zenn-user-upload/0445e8c44e66-20241017.png)

## リファレンス

📍Intel 4004は下記文献をリファレンスにしました🥳

- 📚[Intel 4004 データシート]
   - https://deramp.com/downloads/mfe_archive/050-Component%20Specifications/Intel/Microprocessors%20and%20Support/4004%20Family/4004Data.pdf

- 📚[MCS4 Assembly Language Programming Manual]
   - https://codeabbey.github.io/heavy-data-1/msc-4-asm-manual-1973.pdf

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

### 実装状況

- ✅ ... 実装済み (0/46)
- ❌ ... 未実装 (46/46)
- 📊 ... テスト中 (0/46)

![](https://storage.googleapis.com/zenn-user-upload/5758848744da-20241017.png)

### 転送・分岐命令

| 命令 | オペランド | 説明 |
|------|--------|------|
|❌ JUN | A1A2A3 | Jump Direct / 直接ジャンプ |
|❌ JCN | CCA1A2 | Jump On Conditional / 条件付きジャンプ |
|❌ JIN | RRn | Jump Indirect / 間接ジャンプ |
|❌ ISZ | Rn, A1A2 | Increment Rn and Jump if not zero / レジスタのインクリメントが0でなければジャンプ |
|❌ JMS | A1A2A3 | Jump to Subroutine / サブルーチンへジャンプ |
|❌ BBL | d | Return from Subroutine & d→ACC / サブルーチンから復帰、d->Acc |

### データ転送命令

| 命令 | オペランド | 説明 |
|------|------------|------|
|❌ LD | Rn | Move Rn to Acc. / レジスタからAccへ転送 |
|❌ XCH | Rn | Exchange Rn and Acc / レジスタとAccの交換 |
|❌ STO | Rn | Store Acc to Rn / Accからレジスタにストア |
|❌ LDM | dd | Load Immediate to Acc. / 即値のAccへの読み込み |
|❌ FIM | RRn, dd | Fetch Immediate / 即値のフェッチ |
|❌ FIN | RRn | Fetch Immediate from [RRo]ROM to RRn / ROMから即値をレジスタペアへフェッチ |
|❌ RDM | - | Read RAM to Acc / RAMからAccへ読み込み |
|❌ RD[0:3] | - | Read RAM Reg. [0-3] to Acc. / RAMレジスタ[0-3]からAccへ読み込み |
|❌ RDSGN | - | Read RAM Reg. Sign to Acc / RAM Reg Sign -> Accの読み込み |
|❌ RDDP | - | Read RAM Reg. DP to Acc / RAM Reg DP -> ACCの読み込み |
|❌ RDR | - | Read ROM input part to Acc / ACCにROM読み出し |
|❌ WRM | - | Write Acc. to RAM / Acc -> RAMに書き込み |
|❌ WR[0:3] | - | Write Acc. to RAM Reg. [0-3] / AccからRAMレジスタ[0-3]へ書き込み |
|❌ WRSGN | - | Write ACC. to RAM Reg Sign / Acc -> RAM Reg Signに書き込み |
|❌ WRDP | - | Write ACC. to RAM Reg DP / Acc -> RAM Reg DPに書き込み |
|❌ WRR | - | Write Acc. to ROM output port / AccからROM出力ポートへ書き込み |
|❌ WRP | - | Write Acc. to RAM output port / AccからRAM出力ポートへ書き込み |
|❌ CLDR | - | Clear ALll of RAM and RAM Reg / RAMとRAMレジスタをクリア |
|❌ DSPON | - | Display OutPut is Enabled / 出力の表示有効 |
|❌ DSPOFF | - | Display OutPut is Disabled / 出力の表示無効 |
|❌ RDKB | - | Read input Data on Keyboard Port / データを読み込む |

### アドレスポインタ送出命令

|❌ SRC | RRn | Send Address Pointer RRn to RAM & ROM/ ポインタアドレス送信 |
|❌ DCL | - | Designate Command Line. ACC -> Command Register + RAM select / コマンドライン指定 |

### 算術論理演算命令

| 命令 | オペランド | 説明 |
|------|------------|------|
|❌ ADD | Rn | Add Rn to Accumulator with carry / キャリー付き加算 |
|❌ ADM | - | Add RAM Character to Acc with carry / RAMデータのキャリー付き加算 |
|❌ SUB | Rn | Subtract Rn from Acc. with borrow / ボロー付き減算 |
|❌ SBM | - | Subtract RAM Char. from Acc. with borrow / RAMデータのボロー付き減算 |
|❌ INC | Rn | Increment Rn. / レジスタをインクリメント |
|❌ IAC | - | Increment Acc. / Accのインクリメント |
|❌ DAC | - | Decrement Acc. / Accのデクリメント |
|❌ RAR | - | Rotate Right Acc. with Carry / キャリー付き右回転 |
|❌ RAL | - | Rotate Left Acc. with Carry / キャリー付き左回転 |
|❌ SHR | - | Shift Right Acc / 右シフト |
|❌ SHL | - | Shift Left Acc / 左シフト |
|❌ CLA | - | Clear Acc. / Accのクリア |
|❌ CLB | - | Clear both Acc. and Carry/ Accとキャリーの両方をクリア |
|❌ CMA | - | Complement ACC / Accの補数 |
|❌ STC | - | Set Carry / キャリーフラグのセット |
|❌ CLC | - | Clear Carry / キャリーフラグのクリア |
|❌ CMC | - | Complement Carry / キャリーフラグの補数 |
|❌ TCC | - | Transmit Carry to Acc. then clear Carry / キャリーをAccに転送後クリア |
|❌ DAA | - | Decimal Adjustment for Add / 10進補正 |
|❌ TCS | - | Transfer Carry, then Set / キャリーの転送と設定 |
|❌ KBP | - | Keyboard Process for Code Conversion / キーボード入力の処理 |
|❌ NOP | - | Not Operation / なしもしない |
|❌ HLT | n | HALT for External Signals / 外部信号による停止 |

# 参考文献
🎦動画[ＮＨＫスペシャル　電子立国　日本の自叙伝　第５回　８ミリ角のコンピューター]
- https://www.nhk-ondemand.jp/goods/G2011034631SA000/index.html

📚嶋正利さん著書「マイクロコンピュータの誕生:わが青春の4004」
- https://ci.nii.ac.jp/ncid/BN01286436#anc-library

📚嶋正利さん[マイクロプロセッサ 4004の開発]
- https://www.shmj.or.jp/dev_story/pdf/develop43.pdf

📚[Intel 4004 データシート]
- https://deramp.com/downloads/mfe_archive/050-Component%20Specifications/Intel/Microprocessors%20and%20Support/4004%20Family/4004Data.pdf

📚MCS-4アセンブラのマニュアル[MCS4 Assembly Language Programming Manual]
- https://codeabbey.github.io/heavy-data-1/msc-4-asm-manual-1973.pdf

📚[Intel公式4004関連]
- https://www.intel.co.jp/content/www/jp/ja/newsroom/news/intel-marks-50th-anniversary-4004.html
- https://www.intel.com/content/www/us/en/newsroom/resources/intel-4004.html
- https://www.intel.com/content/www/us/en/history/museum-story-of-intel-4004.html

📚日経[電卓向けの性格を色濃く残す、4004のアーキテクチャー]
- https://xtech.nikkei.com/dm/article/FEATURE/20141212/394135/