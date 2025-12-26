# Autodiff
Rustで実装された自動微分。フォワードモードで微分する。

## 対応している演算
- +, -(単項, 二項) , * , /
- sin, cos, tan, exp, ln

## Test
100% passed

# Log

## diff 関数のシグネチャについて

### 1. diff(f: Fn(Dual) -> Dual, val: f64) -> (f64, f64)

この方式を使うと、呼び出す際に

```rs
diff(|x| 2.into() * x, 3.0)
```

のように、f64 を Dual に変換するコードを書く必要がある。これは冗長であり、ユーザビリティが低下する。

## 解決策
力技。
`impl Add<Dual> for f64`と`impl Add<f64> for Dual`を全てのトレイトに対して行う。
