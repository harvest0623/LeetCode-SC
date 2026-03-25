const MOD = 1000000007

type Fancy struct {
    v []int
    a []int
    b []int
}

func Constructor() Fancy {
    return Fancy{
        v: []int{},
        a: []int{1},
        b: []int{0},
    }
}

func (this *Fancy) Append(val int) {
    this.v = append(this.v, val)
    this.a = append(this.a, this.a[len(this.a)-1])
    this.b = append(this.b, this.b[len(this.b)-1])
}

func (this *Fancy) AddAll(inc int) {
    lastIdx := len(this.b) - 1
    this.b[lastIdx] = (this.b[lastIdx] + inc) % MOD
}

func (this *Fancy) MultAll(m int) {
    lastIdx := len(this.a) - 1
    this.a[lastIdx] = (this.a[lastIdx] * m) % MOD
    this.b[lastIdx] = (this.b[lastIdx] * m) % MOD
}

func (this *Fancy) GetIndex(idx int) int {
    if idx >= len(this.v) {
        return -1
    }
    
    ao := (inv(this.a[idx]) * this.a[len(this.a)-1]) % MOD
    bo := (this.b[len(this.b)-1] - this.b[idx]*ao%MOD + MOD) % MOD
    ans := (ao*this.v[idx]%MOD + bo) % MOD
    return ans
}

// 快速幂
func quickMul(x, y int) int {
    ret := 1
    cur := x
    for y != 0 {
        if y&1 != 0 {
            ret = (ret * cur) % MOD
        }
        cur = (cur * cur) % MOD
        y >>= 1
    }
    return ret
}

// 乘法逆元
func inv(x int) int {
    return quickMul(x, MOD-2)
}