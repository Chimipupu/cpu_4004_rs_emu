# Intel 4004命令セット
📍Intel 4004の命令セットは下記文献をリファレンスにした🥳

📚[Intel 4004 データシート]
   - https://deramp.com/downloads/mfe_archive/050-Component%20Specifications/Intel/Microprocessors%20and%20Support/4004%20Family/4004Data.pdf

📚[MCS4 Assembly Language Programming Manual]
   - https://codeabbey.github.io/heavy-data-1/msc-4-asm-manual-1973.pdf

## 命令セット一覧

| No. | Category                                | Instruction | Description (英語 / 日本語)                                   |
|-----|-----------------------------------------|-------------|----------------------------------------------------------|
| 1   | INDEX REGISTER INSTRUCTIONS             | INC         | Increment index register / インデックスレジスタをインクリメント |
| 2   | INDEX REGISTER INSTRUCTIONS             | FIN         | Finish operation / 操作を終了する                            |
| 3   | INDEX REGISTER TO ACCUMULATOR INSTRUCTIONS | ADD         | Add index register to accumulator / インデックスレジスタをアキュムレータに加算 |
| 4   | INDEX REGISTER TO ACCUMULATOR INSTRUCTIONS | SUB         | Subtract index register from accumulator / アキュムレータからインデックスレジスタを減算 |
| 5   | INDEX REGISTER TO ACCUMULATOR INSTRUCTIONS | LD          | Load index register to accumulator / インデックスレジスタをアキュムレータにロード |
| 6   | INDEX REGISTER TO ACCUMULATOR INSTRUCTIONS | XCH         | Exchange index register with accumulator / インデックスレジスタとアキュムレータを交換 |
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
| 20  | IMMEDIATE INSTRUCTIONS                  | FIM         | Fetch immediate / 即時取得                                    |
| 21  | IMMEDIATE INSTRUCTIONS                  | LDM         | Load accumulator immediate / アキュムレータを即時にロード        |
| 22  | TRANSFER OF CONTROL INSTRUCTIONS        | JUN         | Jump unconditionally / 条件なしジャンプ                         |
| 23  | TRANSFER OF CONTROL INSTRUCTIONS        | JIN         | Jump indirect / 間接ジャンプ                                   |
| 24  | TRANSFER OF CONTROL INSTRUCTIONS        | JCN         | Jump on condition / 条件付きジャンプ                           |
| 25  | TRANSFER OF CONTROL INSTRUCTIONS        | ISZ         | Increment and skip if zero / ゼロの場合はインクリメントしてスキップ |
| 26  | SUBROUTINE LINKAGE INSTRUCTIONS         | JMS         | Jump to subroutine / サブルーチンへジャンプ                    |
| 27  | SUBROUTINE LINKAGE INSTRUCTIONS         | BBL         | Branch back and load / ブランチバックしてロード                  |
| 28  | NOP                                     | NOP         | No operation / ノーオペレーション                               |
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

# INDEX REGISTER INSTRUCTIONS
## INC
![](https://storage.googleapis.com/zenn-user-upload/191f3298cbbf-20241017.png)

## FIN
![](https://storage.googleapis.com/zenn-user-upload/18a0a79dfdd2-20241017.png)

# INDEX REGISTER TO ACCUMULATOR INSTRUCTIONS

## ADD
![](https://storage.googleapis.com/zenn-user-upload/57b96bf053f0-20241017.png)

## SUB
![](https://storage.googleapis.com/zenn-user-upload/3a17985a9d39-20241017.png)

## LD
![](https://storage.googleapis.com/zenn-user-upload/88b2f3b571c5-20241017.png)

## XCH
![](https://storage.googleapis.com/zenn-user-upload/4c2da7b60f51-20241017.png)

# ACCUMULATOR INSTRUCTIONS

## CLB
![](https://storage.googleapis.com/zenn-user-upload/4bb996e67c44-20241017.png)

## CLC
![](https://storage.googleapis.com/zenn-user-upload/85b68a6058d7-20241017.png)

## IAC
![](https://storage.googleapis.com/zenn-user-upload/77fa9d527cf9-20241017.png)

## CMC
![](https://storage.googleapis.com/zenn-user-upload/dc021868c031-20241017.png)

## CMA
![](https://storage.googleapis.com/zenn-user-upload/62b090ad07e9-20241017.png)

## RAL
![](https://storage.googleapis.com/zenn-user-upload/2d5c3367649f-20241017.png)

## RAR
![](https://storage.googleapis.com/zenn-user-upload/fac6410f05c0-20241017.png)

## TCC
![](https://storage.googleapis.com/zenn-user-upload/0ffd5c054052-20241017.png)

## DAC
![](https://storage.googleapis.com/zenn-user-upload/6fb2a01ba871-20241017.png)

## TCS
![](https://storage.googleapis.com/zenn-user-upload/e87389650aa9-20241017.png)

## STC
`STC` (SET CARRY)

![](https://storage.googleapis.com/zenn-user-upload/454eecf66beb-20241017.png)

## DAA
`DAA` (DECIMAL ADJUST ACCUMULATOR)

![](https://storage.googleapis.com/zenn-user-upload/f32b3d5f2054-20241017.png)

## KBP
`KBP` (KEYBOARD PROCESS)

![](https://storage.googleapis.com/zenn-user-upload/8d045b6ff9c1-20241017.png)

# IMMEDIATE INSTRUCTIONS

## FIM
`FIM` (FETCH IMMEDIATE)

![](https://storage.googleapis.com/zenn-user-upload/16a7506b3e22-20241017.png)

## LDM
`LDM`(LOAD ACCUMULATOR IMMEDIATE)

![](https://storage.googleapis.com/zenn-user-upload/04b772a2116b-20241017.png)

# TRANSFER OF CONTROL INSTRUCTIONS

## JUN
`JUN` (JUMP UNCONDITIONALLY)

![](https://storage.googleapis.com/zenn-user-upload/22d85bc0afe5-20241017.png)

## JIN
`JIN`(JUMP INDIRECT)

![](https://storage.googleapis.com/zenn-user-upload/bbdadbbb5405-20241017.png)

## JCN
`JCN`(JUMP ON CONDITION)

![](https://storage.googleapis.com/zenn-user-upload/a9f34254940d-20241017.png)
![](https://storage.googleapis.com/zenn-user-upload/46ea7d1d6a36-20241017.png)

## ISZ
`ISZ`(INCREMENT AND SKIP IF ZERO)

![](https://storage.googleapis.com/zenn-user-upload/7df8226a6ab4-20241017.png)

# SUBROUTINE LINKAGE INSTRUCTIONS

## JMS
`JMS`(JUMP TO SUBROUTINE)

![](https://storage.googleapis.com/zenn-user-upload/0617c8c1a99e-20241017.png)

## BBL
`BBL`(BRANCH BACK AND LOAD)

![](https://storage.googleapis.com/zenn-user-upload/c73d8f62ee49-20241017.png)

# NOP
![](https://storage.googleapis.com/zenn-user-upload/0445380edec5-20241017.png)

# MEMORY SELECTION INSTRUCTIONS
## DCL
`DCL`(DESIGNATE COMMAND LINE)

![](https://storage.googleapis.com/zenn-user-upload/7c5ce74f1c3c-20241017.png)

## SRC
`SRC`(SEND REGISTER CONTROL)

![](https://storage.googleapis.com/zenn-user-upload/7d180665408b-20241017.png)

# I/O AND RAM INSTRUCTIONS

## WRM
`WRM`(WRITE DATA RAM CHARACTER)

![](https://storage.googleapis.com/zenn-user-upload/0af10a363a5a-20241017.png)

## WMP
`WMP`(WRITE RAM PORT)

![](https://storage.googleapis.com/zenn-user-upload/a8b42ac91f5b-20241017.png)

## WRR
`WRR`(WRITE ROM PORT)

![](https://storage.googleapis.com/zenn-user-upload/83c1cb1c858e-20241017.png)

## WPM
`WPM`(WRITE P ROG RAM RAM)

![](https://storage.googleapis.com/zenn-user-upload/5208dfe49250-20241017.png)

## WRn
`WRn`(WRITE DATA RAM STATUS CHARACTER)

![](https://storage.googleapis.com/zenn-user-upload/955bdeb521b2-20241017.png)

## RDM
`RDM`(READ DATA RAM DATA CHARACTER)

![](https://storage.googleapis.com/zenn-user-upload/62c48f91b66e-20241017.png)

## RDR
`RDR`(READ ROM PORT)

![](https://storage.googleapis.com/zenn-user-upload/7a59cd3dfc1a-20241017.png)

## RDn
`RDn`(READ DATA RAM STATUS CHARACTER)

![](https://storage.googleapis.com/zenn-user-upload/71d3694dbf02-20241017.png)

## ADM
`ADM`(ADD DATA RAM TO ACCUMULATOR WITH CARRY)
![](https://storage.googleapis.com/zenn-user-upload/cdc17d474f8d-20241017.png)

## SBM
`SBM`(SUBTRACT DATA RAM FROM MEMORY WITH BORROV\T)
![](https://storage.googleapis.com/zenn-user-upload/b432dcc9da3b-20241017.png)