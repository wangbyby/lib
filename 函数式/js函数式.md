>
- 纯函数是这样一种函数，即相同的输入，永远会得到相同的输出，而且没有任何可观察的副作用。

- 副作用是在计算结果的过程中，系统状态的一种变化，或者与外部世界进行的可观察的交互。

副作用可能包含，但不限于：
- 更改文件系统
- 往数据库插入记录
- 发送一个 http 请求
- 可变数据
- 打印/log
- 获取用户输入
- DOM 查询
- 访问系统状态
``函数式编程的哲学就是假定副作用是造成不正当行为的主要原因。``

副作用让一个函数变得不纯是有道理的：从定义上来说，纯函数必须要能够根据相同的输入返回相同的输出；如果函数需要跟外部事物打交道，那么就无法保证这一点了。

``纯函数就是数学上的函数，而且是函数式编程的全部。使用这些纯函数编程能够带来大量的好处，让我们来看一下为何要不遗余力地保留函数的纯粹性的原因。``

# 追求纯的理由
## 可缓存性
实现缓存的一种典型方式是 memoize 技术：

## 可移植性／自文档化
## 可测试性
## 合理性
等式推导带来的分析代码的能力对重构和理解代码非常重要
## 并行代码

# 柯里化 curry 7局部调用（partial application）
``curry 的概念很简单：只传递给函数一部分参数来调用它，让它返回一个函数去处理剩下的参数。``
只传给函数一部分参数通常也叫做局部调用（partial application），能够大量减少样板文件代码（boilerplate code）。
```Javascript
    var _ = require('ramda');

    var map = _(function(f, ary) {
    return ary.map(f);
    });

    // 练习 1
    //==============
    // 通过局部调用（partial apply）移除所有参数

    // var words = function(str) {
    //     return split(' ', str);
    // };
    var split = _(function(what, str) { 
            return  split(what, str); 
    }); 
    var words = split(' '); // 

    // 练习 1a
    //==============
    // 使用 `map` 创建一个新的 `words` 函数，使之能够操作字符串数组
    // var getEle = function(x){ return x} ;
    var sentences = map(words);


    // 练习 2
    //==============
    // 通过局部调用（partial apply）移除所有参数

    
    // var filterQs = function(xs) {
    //     return filter(function(x){ return match(/q/i, x);  }, xs);
    // };
    
    var match_what = match(/q/i);
    var finds = filter(match_what);
    var filterQs = finds;

    // 练习 3
    //==============
    // 使用帮助函数 `_keepHighest` 重构 `max` 使之成为 curry 函数

    // 无须改动:
    var _keepHighest = function(x,y){ return x >= y ? x : y; };

    // 重构这段代码:
    // var max = function(xs) {
    //     return reduce(
    //                 function(acc, x){return _keepHighest(acc, x);},
    //                 -Infinity, 
    //                 xs);
    // };

    var  __max = curry(
        _keepHighest
    );
    var max = __max(-Infinity);

    // 彩蛋 1:
    // ============
    // 包裹数组的 `slice` 函数使之成为 curry 函数
    // //[1,2,3].slice(0, 2)
    var slice = curry( function(l,r,arr)
        return arr.slice(l,r);
    );
    
    // 彩蛋 2:
    // ============
    // 借助 `slice` 定义一个 `take` curry 函数，该函数调用后可以取出字符串的前 n 个字符。
    var take = slice(0,n);


```

# 组合& pointfree
```Javascript
var compose = function(f,g) {
  return function(x) {
    return f(g(x));
  };
};
```

## 范畴学
在范畴学中，有一个概念叫做范畴。有着以下这些组件component的搜集collection就构成了一个范畴：
- 对象的搜集
- 态射的搜集
- 态射的组合        
  - 结合律是在范畴学中对任何组合都适用的一个特性
- identity 这个独特的态射

```Javascript
require('../../support');
var _ = require('ramda');
var accounting = require('accounting');

// 示例数据
var CARS = [
    {name: "Ferrari FF", horsepower: 660, dollar_value: 700000, in_stock: true},
    {name: "Spyker C12 Zagato", horsepower: 650, dollar_value: 648000, in_stock: false},
    {name: "Jaguar XKR-S", horsepower: 550, dollar_value: 132000, in_stock: false},
    {name: "Audi R8", horsepower: 525, dollar_value: 114200, in_stock: false},
    {name: "Aston Martin One-77", horsepower: 750, dollar_value: 1850000, in_stock: true},
    {name: "Pagani Huayra", horsepower: 700, dollar_value: 1300000, in_stock: false}
  ];

// 练习 1:
// ============
// 使用 _.compose() 重写下面这个函数。提示：_.prop() 是 curry 函数
// var isLastInStock = function(cars) {
//   var last_car = _.last(cars);
//   return _.prop('in_stock', last_car);
// };

var isLastInStock = _.compose( _.prop('in_stock') , _.last );

// 练习 2:
// ============
// 使用 _.compose()、_.prop() 和 _.head() 获取第一个 car 的 name
var nameOfFirstCar =  _.compose( _.prop('name') , _.head );


// 练习 3:
// ============
// 使用帮助函数 _average 重构 averageDollarValue 使之成为一个组合
var _average = function(xs) { return reduce(add, 0, xs) / xs.length; }; // <- 无须改动

// var averageDollarValue = function(cars) {
//   var dollar_values = map(function(c) { return c.dollar_value; }, cars);
//   return _average(dollar_values);
// };
var dollar = map(function(c){return c.dollar_value;});
var averageDollarValue = _.compose( dollar , _average);


// 练习 4:
// ============
// 使用 compose 写一个 sanitizeNames() 函数，返回一个下划线连接的小写字符串：例如：sanitizeNames(["Hello World"]) //=> ["hello_world"]。

var _underscore = replace(/\W+/g, '_'); //<-- 无须改动，并在 sanitizeNames 中使用它

var sanitizeNames = compose(_underscore, toLowerCase , map);


// 彩蛋 1:
// ============
// 使用 compose 重构 availablePrices

// var availablePrices = function(cars) {
//   var available_cars = _.filter(_.prop('in_stock'), cars);
//   return available_cars.map(function(x){
//     return accounting.formatMoney(x.dollar_value);
//   }).join(', ');
// };

var availablePrices = compose(_.filter(_.prop('in_stock'))  , compose(join(', '), map(formatMoney) ));

// 彩蛋 2:
// ============
// 重构使之成为 pointfree 函数。提示：可以使用 _.flip()

var fastestCar = function(cars) {
  var sorted = _.sortBy(function(car){ return car.horsepower }, cars);
  var fastest = _.last(sorted);
  return fastest.name + ' is the fastest';
};

var fastestCar = compose( _.sortBy(function(car){ return car.horsepower }), _.last  );
```

# 例子

