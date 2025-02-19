# うさぎとキツネの個体数シミュレーション

- $R$: うさぎの個体数
- $F$: キツネの個体数
- $t$: 時間
- $B_r$: うさぎの出生率
- $D_r$: うさぎの死亡率
- $B_f$: キツネの出生率
- $D_f$: キツネの死亡率

## 個体数のモデル化

### 個体数

- $R(t + \Delta t) - R(t) = (B_r - D_r)R(t)\Delta t$
- $F(t + \Delta t) - F(t) = (B_f - D_f)F(t)\Delta t$

### 死亡率と個体数に関する仮定

- $D_r = aF(t)$
- $B_f = bR(t)$

### シミュレーションの数理モデル

- $R(t + \Delta t) = (1 + (B_r - aF(t)) \Delta t)R(t)$
- $F(t + \Delta t) = (1 + (bR(t) - D_f) \Delta t)F(t)$
