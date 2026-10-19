## 难点：闭包
作为已经有rust基础的学生，在写并发练习的时候，最容易感到困惑的不是并发本身，反而是关于闭包的语法

举个例子
```rust
thread_local! {
    static COUNTER: RefCell<usize> = RefCell::new(0);
}

fn increment_thread_local()-> usize{
    COUNTER.with(|counter| {
        let mut value = counter.borrow_mut();
        *value += 1;
        *value
    })
}
``` 
困难的地方在于和闭包打交道的函数方法和正常的`链式调用的读法不太一样

对于上面一个例子，我们的注意力暂且只需要关注: COUNTER.with() 这一行

按照 rust 函数命名的习惯，我们知道with的方法通常传入的是一个闭包

让我们来对照一下

**有闭包**
```rust
COUNTER.with(|counter|{...})
```
**无闭包**
```rust
let mut a =1;
a.add(1);
```

形式上，都是套用的同一个模板，变量类型 + 方法名 + 括号，似乎读法一致，都是看顿号之前的返回值类型，决定下一个方法的传入什么

例如 a.add(1) 其实等价于 add(a, 1), 把 a 放入 add 方法调用的形式参数列表当中。按照这样推理，应该 COUNTER.with()方法也可以读成 with(COUNTER, 闭包)。

到这里没有什么问题，然后我们会想当然的把 COUNTER 赋值给闭包中的形式参数 counter了。

---

然而事实并非如此，当闭包方法作为形式参数的时候，那么链式法则就已经失效了，不然把闭包的输入看作上一段的返回值。

那么你可能会疑惑，那闭包的输入值是什么？

这个问题是难点，也是闭包特别不透明的一点，他藏在函数签名中

```rust
impl<T: 'static> LocalKey<T>{
  fn with<F, R>(&'static self,f: F) -> R
  where F: FnOnce(&T) -> R
  {

  }
}
```

你会发现非常变态的一点就是，传递给闭包的输入参数是&T, 也就是 LocalKey类型的 COUNTER 变量包裹的那个RefCell(0)，这个才是T

也就是说，传递给闭包形式参数 counter 的是 &RefCell(0)

而不是是self类型本身(COUNTER 变量的 LocalKey类型)，因此你不难看出为啥链式法则失效了

闭包的输入类型取决于 trait，而不是他被传入了哪个类型的方法调用中
