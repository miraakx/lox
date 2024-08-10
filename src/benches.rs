pub const INVOCATION_LOX: &str = "class Foo {
  method0() {}
  method1() {}
  method2() {}
  method3() {}
  method4() {}
  method5() {}
  method6() {}
  method7() {}
  method8() {}
  method9() {}
  method10() {}
  method11() {}
  method12() {}
  method13() {}
  method14() {}
  method15() {}
  method16() {}
  method17() {}
  method18() {}
  method19() {}
  method20() {}
  method21() {}
  method22() {}
  method23() {}
  method24() {}
  method25() {}
  method26() {}
  method27() {}
  method28() {}
  method29() {}
}
var foo = Foo();
var start = clock();
var i = 0;
while (i < 500000) {
  foo.method0();
  foo.method1();
  foo.method2();
  foo.method3();
  foo.method4();
  foo.method5();
  foo.method6();
  foo.method7();
  foo.method8();
  foo.method9();
  foo.method10();
  foo.method11();
  foo.method12();
  foo.method13();
  foo.method14();
  foo.method15();
  foo.method16();
  foo.method17();
  foo.method18();
  foo.method19();
  foo.method20();
  foo.method21();
  foo.method22();
  foo.method23();
  foo.method24();
  foo.method25();
  foo.method26();
  foo.method27();
  foo.method28();
  foo.method29();
  i = i + 1;
}
print \"elapsed: \";
print clock() - start;";
pub const PROPERTIES_LOX: &str = "class Foo {
  init() {
    this.field0 = 1;
    this.field1 = 1;
    this.field2 = 1;
    this.field3 = 1;
    this.field4 = 1;
    this.field5 = 1;
    this.field6 = 1;
    this.field7 = 1;
    this.field8 = 1;
    this.field9 = 1;
    this.field10 = 1;
    this.field11 = 1;
    this.field12 = 1;
    this.field13 = 1;
    this.field14 = 1;
    this.field15 = 1;
    this.field16 = 1;
    this.field17 = 1;
    this.field18 = 1;
    this.field19 = 1;
    this.field20 = 1;
    this.field21 = 1;
    this.field22 = 1;
    this.field23 = 1;
    this.field24 = 1;
    this.field25 = 1;
    this.field26 = 1;
    this.field27 = 1;
    this.field28 = 1;
    this.field29 = 1;
  }
  method0() { return this.field0; }
  method1() { return this.field1; }
  method2() { return this.field2; }
  method3() { return this.field3; }
  method4() { return this.field4; }
  method5() { return this.field5; }
  method6() { return this.field6; }
  method7() { return this.field7; }
  method8() { return this.field8; }
  method9() { return this.field9; }
  method10() { return this.field10; }
  method11() { return this.field11; }
  method12() { return this.field12; }
  method13() { return this.field13; }
  method14() { return this.field14; }
  method15() { return this.field15; }
  method16() { return this.field16; }
  method17() { return this.field17; }
  method18() { return this.field18; }
  method19() { return this.field19; }
  method20() { return this.field20; }
  method21() { return this.field21; }
  method22() { return this.field22; }
  method23() { return this.field23; }
  method24() { return this.field24; }
  method25() { return this.field25; }
  method26() { return this.field26; }
  method27() { return this.field27; }
  method28() { return this.field28; }
  method29() { return this.field29; }
}
var foo = Foo();
var start = clock();
var i = 0;
while (i < 500000) {
  foo.method0();
  foo.method1();
  foo.method2();
  foo.method3();
  foo.method4();
  foo.method5();
  foo.method6();
  foo.method7();
  foo.method8();
  foo.method9();
  foo.method10();
  foo.method11();
  foo.method12();
  foo.method13();
  foo.method14();
  foo.method15();
  foo.method16();
  foo.method17();
  foo.method18();
  foo.method19();
  foo.method20();
  foo.method21();
  foo.method22();
  foo.method23();
  foo.method24();
  foo.method25();
  foo.method26();
  foo.method27();
  foo.method28();
  foo.method29();
  i = i + 1;
}
print \"elapsed: \";
print clock() - start;";
pub const INSTANTIATION_LOX: &str = "class Foo {
  init() {}
}
var start = clock();
var i = 0;
while (i < 500000) {
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  i = i + 1;
}
print \"elapsed: \";
print clock() - start;";
pub const ZOO_BATCH_LOX: &str = "class Zoo {
  init() {
    this.aarvark  = 1;
    this.baboon   = 1;
    this.cat      = 1;
    this.donkey   = 1;
    this.elephant = 1;
    this.fox      = 1;
  }
  ant()    { return this.aarvark; }
  banana() { return this.baboon; }
  tuna()   { return this.cat; }
  hay()    { return this.donkey; }
  grass()  { return this.elephant; }
  mouse()  { return this.fox; }
}
var zoo = Zoo();
var sum = 0;
var start = clock();
var batch = 0;
while (clock() - start < 10) {
  for (var i = 0; i < 10000; i = i + 1) {
    sum = sum + zoo.ant()
              + zoo.banana()
              + zoo.tuna()
              + zoo.hay()
              + zoo.grass()
              + zoo.mouse();
  }
  batch = batch + 1;
}
print \"elapsed: \";
print clock() - start;";
pub const METHOD_CALL_LOX: &str = "class Toggle {
  init(startState) {
    this.state = startState;
  }
  value() { return this.state; }
  activate() {
    this.state = !this.state;
    return this;
  }
}
class NthToggle < Toggle {
  init(startState, maxCounter) {
    super.init(startState);
    this.countMax = maxCounter;
    this.count = 0;
  }
  activate() {
    this.count = this.count + 1;
    if (this.count >= this.countMax) {
      super.activate();
      this.count = 0;
    }
    return this;
  }
}
var start = clock();
var n = 100000;
var val = true;
var toggle = Toggle(val);
for (var i = 0; i < n; i = i + 1) {
  val = toggle.activate().value();
  val = toggle.activate().value();
  val = toggle.activate().value();
  val = toggle.activate().value();
  val = toggle.activate().value();
  val = toggle.activate().value();
  val = toggle.activate().value();
  val = toggle.activate().value();
  val = toggle.activate().value();
  val = toggle.activate().value();
}
val = true;
var ntoggle = NthToggle(val, 3);
for (var i = 0; i < n; i = i + 1) {
  val = ntoggle.activate().value();
  val = ntoggle.activate().value();
  val = ntoggle.activate().value();
  val = ntoggle.activate().value();
  val = ntoggle.activate().value();
  val = ntoggle.activate().value();
  val = ntoggle.activate().value();
  val = ntoggle.activate().value();
  val = ntoggle.activate().value();
  val = ntoggle.activate().value();
}
print \"elapsed: \";
print clock() - start;";
pub const TREES_LOX: &str = "class Tree {
  init(depth) {
    this.depth = depth;
    if (depth > 0) {
      this.a = Tree(depth - 1);
      this.b = Tree(depth - 1);
      this.c = Tree(depth - 1);
      this.d = Tree(depth - 1);
      this.e = Tree(depth - 1);
    }
  }
  walk() {
    if (this.depth == 0) return 0;
    return this.depth
        + this.a.walk()
        + this.b.walk()
        + this.c.walk()
        + this.d.walk()
        + this.e.walk();
  }
}
var tree = Tree(8);
var start = clock();
for (var i = 0; i < 100; i = i + 1) {
  if (tree.walk() != 122068) print \"Error\";
}
print \"elapsed: \";
print clock() - start;";
pub const EQUALITY_LOX: &str = "var i = 0;
var loopStart = clock();
while (i < 10000000) {
  i = i + 1;
  1; 1; 1; 2; 1; nil; 1; \"str\"; 1; true;
  nil; nil; nil; 1; nil; \"str\"; nil; true;
  true; true; true; 1; true; false; true; \"str\"; true; nil;
  \"str\"; \"str\"; \"str\"; \"stru\"; \"str\"; 1; \"str\"; nil; \"str\"; true;
}
var loopTime = clock() - loopStart;
i = 0;
while (i < 10000000) {
  i = i + 1;
  1 == 1; 1 == 2; 1 == nil; 1 == \"str\"; 1 == true;
  nil == nil; nil == 1; nil == \"str\"; nil == true;
  true == true; true == 1; true == false; true == \"str\"; true == nil;
  \"str\" == \"str\"; \"str\" == \"stru\"; \"str\" == 1; \"str\" == nil; \"str\" == true;
}
var elapsed = clock() - loopStart;
print \"elapsed: \";
print elapsed - loopTime;";
pub const BINARY_TREES_LOX: &str = "class Tree {
  init(item, depth) {
    this.item = item;
    this.depth = depth;
    if (depth > 0) {
      var item2 = item + item;
      depth = depth - 1;
      this.left = Tree(item2 - 1, depth);
      this.right = Tree(item2, depth);
    } else {
      this.left = nil;
      this.right = nil;
    }
  }
  check() {
    if (this.left == nil) {
      return this.item;
    }
    return this.item + this.left.check() - this.right.check();
  }
}
var minDepth = 4;
var maxDepth = 14;
var stretchDepth = maxDepth + 1;
var start = clock();
var longLivedTree = Tree(0, maxDepth);
var iterations = 1;
var d = 0;
while (d < maxDepth) {
  iterations = iterations * 2;
  d = d + 1;
}
var depth = minDepth;
while (depth < stretchDepth) {
  var check = 0;
  var i = 1;
  while (i <= iterations) {
    check = check + Tree(i, depth).check() + Tree(-i, depth).check();
    i = i + 1;
  }
  iterations = iterations / 4;
  depth = depth + 2;
}
print \"elapsed:\";
print clock() - start;";
pub const ZOO_LOX: &str = "class Zoo {
  init() {
    this.aarvark  = 1;
    this.baboon   = 1;
    this.cat      = 1;
    this.donkey   = 1;
    this.elephant = 1;
    this.fox      = 1;
  }
  ant()    { return this.aarvark; }
  banana() { return this.baboon; }
  tuna()   { return this.cat; }
  hay()    { return this.donkey; }
  grass()  { return this.elephant; }
  mouse()  { return this.fox; }
}

var zoo = Zoo();
var sum = 0;
var start = clock();
while (sum < 10000000) {
  sum = sum + zoo.ant()
            + zoo.banana()
            + zoo.tuna()
            + zoo.hay()
            + zoo.grass()
            + zoo.mouse();
}
print \"elapsed: \";
print clock() - start;";
pub const STRING_EQUALITY_LOX: &str = "var a1 = \"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa1\";
var a2 = \"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa2\";
var a3 = \"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa3\";
var a4 = \"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa4\";
var a5 = \"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa5\";
var a6 = \"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa6\";
var a7 = \"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa7\";
var a8 = \"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa8\";
var i = 0;
var loopStart = clock();
while (i < 100000) {
  i = i + 1;
  a1; a1; a1; a2; a1; a3; a1; a4; a1; a5; a1; a6; a1; a7; a1; a8;
  a2; a1; a2; a2; a2; a3; a2; a4; a2; a5; a2; a6; a2; a7; a2; a8;
  a3; a1; a3; a2; a3; a3; a3; a4; a3; a5; a3; a6; a3; a7; a3; a8;
  a4; a1; a4; a2; a4; a3; a4; a4; a4; a5; a4; a6; a4; a7; a4; a8;
  a5; a1; a5; a2; a5; a3; a5; a4; a5; a5; a5; a6; a5; a7; a5; a8;
  a6; a1; a6; a2; a6; a3; a6; a4; a6; a5; a6; a6; a6; a7; a6; a8;
  a7; a1; a7; a2; a7; a3; a7; a4; a7; a5; a7; a6; a7; a7; a7; a8;
  a8; a1; a8; a2; a8; a3; a8; a4; a8; a5; a8; a6; a8; a7; a8; a8;
  a1; a1; a1; a2; a1; a3; a1; a4; a1; a5; a1; a6; a1; a7; a1; a8;
  a2; a1; a2; a2; a2; a3; a2; a4; a2; a5; a2; a6; a2; a7; a2; a8;
  a3; a1; a3; a2; a3; a3; a3; a4; a3; a5; a3; a6; a3; a7; a3; a8;
  a4; a1; a4; a2; a4; a3; a4; a4; a4; a5; a4; a6; a4; a7; a4; a8;
  a5; a1; a5; a2; a5; a3; a5; a4; a5; a5; a5; a6; a5; a7; a5; a8;
  a6; a1; a6; a2; a6; a3; a6; a4; a6; a5; a6; a6; a6; a7; a6; a8;
  a7; a1; a7; a2; a7; a3; a7; a4; a7; a5; a7; a6; a7; a7; a7; a8;
  a8; a1; a8; a2; a8; a3; a8; a4; a8; a5; a8; a6; a8; a7; a8; a8;
  a1; a1; a1; a2; a1; a3; a1; a4; a1; a5; a1; a6; a1; a7; a1; a8;
  a2; a1; a2; a2; a2; a3; a2; a4; a2; a5; a2; a6; a2; a7; a2; a8;
  a3; a1; a3; a2; a3; a3; a3; a4; a3; a5; a3; a6; a3; a7; a3; a8;
  a4; a1; a4; a2; a4; a3; a4; a4; a4; a5; a4; a6; a4; a7; a4; a8;
  a5; a1; a5; a2; a5; a3; a5; a4; a5; a5; a5; a6; a5; a7; a5; a8;
  a6; a1; a6; a2; a6; a3; a6; a4; a6; a5; a6; a6; a6; a7; a6; a8;
  a7; a1; a7; a2; a7; a3; a7; a4; a7; a5; a7; a6; a7; a7; a7; a8;
  a8; a1; a8; a2; a8; a3; a8; a4; a8; a5; a8; a6; a8; a7; a8; a8;
  a1; a1; a1; a2; a1; a3; a1; a4; a1; a5; a1; a6; a1; a7; a1; a8;
  a2; a1; a2; a2; a2; a3; a2; a4; a2; a5; a2; a6; a2; a7; a2; a8;
  a3; a1; a3; a2; a3; a3; a3; a4; a3; a5; a3; a6; a3; a7; a3; a8;
  a4; a1; a4; a2; a4; a3; a4; a4; a4; a5; a4; a6; a4; a7; a4; a8;
  a5; a1; a5; a2; a5; a3; a5; a4; a5; a5; a5; a6; a5; a7; a5; a8;
  a6; a1; a6; a2; a6; a3; a6; a4; a6; a5; a6; a6; a6; a7; a6; a8;
  a7; a1; a7; a2; a7; a3; a7; a4; a7; a5; a7; a6; a7; a7; a7; a8;
  a8; a1; a8; a2; a8; a3; a8; a4; a8; a5; a8; a6; a8; a7; a8; a8;
  a1; a1; a1; a2; a1; a3; a1; a4; a1; a5; a1; a6; a1; a7; a1; a8;
  a2; a1; a2; a2; a2; a3; a2; a4; a2; a5; a2; a6; a2; a7; a2; a8;
  a3; a1; a3; a2; a3; a3; a3; a4; a3; a5; a3; a6; a3; a7; a3; a8;
  a4; a1; a4; a2; a4; a3; a4; a4; a4; a5; a4; a6; a4; a7; a4; a8;
  a5; a1; a5; a2; a5; a3; a5; a4; a5; a5; a5; a6; a5; a7; a5; a8;
  a6; a1; a6; a2; a6; a3; a6; a4; a6; a5; a6; a6; a6; a7; a6; a8;
  a7; a1; a7; a2; a7; a3; a7; a4; a7; a5; a7; a6; a7; a7; a7; a8;
  a8; a1; a8; a2; a8; a3; a8; a4; a8; a5; a8; a6; a8; a7; a8; a8;
  a1; a1; a1; a2; a1; a3; a1; a4; a1; a5; a1; a6; a1; a7; a1; a8;
  a2; a1; a2; a2; a2; a3; a2; a4; a2; a5; a2; a6; a2; a7; a2; a8;
  a3; a1; a3; a2; a3; a3; a3; a4; a3; a5; a3; a6; a3; a7; a3; a8;
  a4; a1; a4; a2; a4; a3; a4; a4; a4; a5; a4; a6; a4; a7; a4; a8;
  a5; a1; a5; a2; a5; a3; a5; a4; a5; a5; a5; a6; a5; a7; a5; a8;
  a6; a1; a6; a2; a6; a3; a6; a4; a6; a5; a6; a6; a6; a7; a6; a8;
  a7; a1; a7; a2; a7; a3; a7; a4; a7; a5; a7; a6; a7; a7; a7; a8;
  a8; a1; a8; a2; a8; a3; a8; a4; a8; a5; a8; a6; a8; a7; a8; a8;
  a1; a1; a1; a2; a1; a3; a1; a4; a1; a5; a1; a6; a1; a7; a1; a8;
  a2; a1; a2; a2; a2; a3; a2; a4; a2; a5; a2; a6; a2; a7; a2; a8;
  a3; a1; a3; a2; a3; a3; a3; a4; a3; a5; a3; a6; a3; a7; a3; a8;
  a4; a1; a4; a2; a4; a3; a4; a4; a4; a5; a4; a6; a4; a7; a4; a8;
  a5; a1; a5; a2; a5; a3; a5; a4; a5; a5; a5; a6; a5; a7; a5; a8;
  a6; a1; a6; a2; a6; a3; a6; a4; a6; a5; a6; a6; a6; a7; a6; a8;
  a7; a1; a7; a2; a7; a3; a7; a4; a7; a5; a7; a6; a7; a7; a7; a8;
  a8; a1; a8; a2; a8; a3; a8; a4; a8; a5; a8; a6; a8; a7; a8; a8;
  a1; a1; a1; a2; a1; a3; a1; a4; a1; a5; a1; a6; a1; a7; a1; a8;
  a2; a1; a2; a2; a2; a3; a2; a4; a2; a5; a2; a6; a2; a7; a2; a8;
  a3; a1; a3; a2; a3; a3; a3; a4; a3; a5; a3; a6; a3; a7; a3; a8;
  a4; a1; a4; a2; a4; a3; a4; a4; a4; a5; a4; a6; a4; a7; a4; a8;
  a5; a1; a5; a2; a5; a3; a5; a4; a5; a5; a5; a6; a5; a7; a5; a8;
  a6; a1; a6; a2; a6; a3; a6; a4; a6; a5; a6; a6; a6; a7; a6; a8;
  a7; a1; a7; a2; a7; a3; a7; a4; a7; a5; a7; a6; a7; a7; a7; a8;
  a8; a1; a8; a2; a8; a3; a8; a4; a8; a5; a8; a6; a8; a7; a8; a8;
  a1; a1; a1; a2; a1; a3; a1; a4; a1; a5; a1; a6; a1; a7; a1; a8;
  a2; a1; a2; a2; a2; a3; a2; a4; a2; a5; a2; a6; a2; a7; a2; a8;
  a3; a1; a3; a2; a3; a3; a3; a4; a3; a5; a3; a6; a3; a7; a3; a8;
  a4; a1; a4; a2; a4; a3; a4; a4; a4; a5; a4; a6; a4; a7; a4; a8;
  a5; a1; a5; a2; a5; a3; a5; a4; a5; a5; a5; a6; a5; a7; a5; a8;
  a6; a1; a6; a2; a6; a3; a6; a4; a6; a5; a6; a6; a6; a7; a6; a8;
  a7; a1; a7; a2; a7; a3; a7; a4; a7; a5; a7; a6; a7; a7; a7; a8;
  a8; a1; a8; a2; a8; a3; a8; a4; a8; a5; a8; a6; a8; a7; a8; a8;
  a1; a1; a1; a2; a1; a3; a1; a4; a1; a5; a1; a6; a1; a7; a1; a8;
  a2; a1; a2; a2; a2; a3; a2; a4; a2; a5; a2; a6; a2; a7; a2; a8;
  a3; a1; a3; a2; a3; a3; a3; a4; a3; a5; a3; a6; a3; a7; a3; a8;
  a4; a1; a4; a2; a4; a3; a4; a4; a4; a5; a4; a6; a4; a7; a4; a8;
  a5; a1; a5; a2; a5; a3; a5; a4; a5; a5; a5; a6; a5; a7; a5; a8;
  a6; a1; a6; a2; a6; a3; a6; a4; a6; a5; a6; a6; a6; a7; a6; a8;
  a7; a1; a7; a2; a7; a3; a7; a4; a7; a5; a7; a6; a7; a7; a7; a8;
  a8; a1; a8; a2; a8; a3; a8; a4; a8; a5; a8; a6; a8; a7; a8; a8;
}
var loopTime = clock() - loopStart;
var start = clock();
i = 0;
while (i < 100000) {
  i = i + 1;
  a1 == a1; a1 == a2; a1 == a3; a1 == a4; a1 == a5; a1 == a6; a1 == a7; a1 == a8;
  a2 == a1; a2 == a2; a2 == a3; a2 == a4; a2 == a5; a2 == a6; a2 == a7; a2 == a8;
  a3 == a1; a3 == a2; a3 == a3; a3 == a4; a3 == a5; a3 == a6; a3 == a7; a3 == a8;
  a4 == a1; a4 == a2; a4 == a3; a4 == a4; a4 == a5; a4 == a6; a4 == a7; a4 == a8;
  a5 == a1; a5 == a2; a5 == a3; a5 == a4; a5 == a5; a5 == a6; a5 == a7; a5 == a8;
  a6 == a1; a6 == a2; a6 == a3; a6 == a4; a6 == a5; a6 == a6; a6 == a7; a6 == a8;
  a7 == a1; a7 == a2; a7 == a3; a7 == a4; a7 == a5; a7 == a6; a7 == a7; a7 == a8;
  a8 == a1; a8 == a2; a8 == a3; a8 == a4; a8 == a5; a8 == a6; a8 == a7; a8 == a8;
  a1 == a1; a1 == a2; a1 == a3; a1 == a4; a1 == a5; a1 == a6; a1 == a7; a1 == a8;
  a2 == a1; a2 == a2; a2 == a3; a2 == a4; a2 == a5; a2 == a6; a2 == a7; a2 == a8;
  a3 == a1; a3 == a2; a3 == a3; a3 == a4; a3 == a5; a3 == a6; a3 == a7; a3 == a8;
  a4 == a1; a4 == a2; a4 == a3; a4 == a4; a4 == a5; a4 == a6; a4 == a7; a4 == a8;
  a5 == a1; a5 == a2; a5 == a3; a5 == a4; a5 == a5; a5 == a6; a5 == a7; a5 == a8;
  a6 == a1; a6 == a2; a6 == a3; a6 == a4; a6 == a5; a6 == a6; a6 == a7; a6 == a8;
  a7 == a1; a7 == a2; a7 == a3; a7 == a4; a7 == a5; a7 == a6; a7 == a7; a7 == a8;
  a8 == a1; a8 == a2; a8 == a3; a8 == a4; a8 == a5; a8 == a6; a8 == a7; a8 == a8;
  a1 == a1; a1 == a2; a1 == a3; a1 == a4; a1 == a5; a1 == a6; a1 == a7; a1 == a8;
  a2 == a1; a2 == a2; a2 == a3; a2 == a4; a2 == a5; a2 == a6; a2 == a7; a2 == a8;
  a3 == a1; a3 == a2; a3 == a3; a3 == a4; a3 == a5; a3 == a6; a3 == a7; a3 == a8;
  a4 == a1; a4 == a2; a4 == a3; a4 == a4; a4 == a5; a4 == a6; a4 == a7; a4 == a8;
  a5 == a1; a5 == a2; a5 == a3; a5 == a4; a5 == a5; a5 == a6; a5 == a7; a5 == a8;
  a6 == a1; a6 == a2; a6 == a3; a6 == a4; a6 == a5; a6 == a6; a6 == a7; a6 == a8;
  a7 == a1; a7 == a2; a7 == a3; a7 == a4; a7 == a5; a7 == a6; a7 == a7; a7 == a8;
  a8 == a1; a8 == a2; a8 == a3; a8 == a4; a8 == a5; a8 == a6; a8 == a7; a8 == a8;
  a1 == a1; a1 == a2; a1 == a3; a1 == a4; a1 == a5; a1 == a6; a1 == a7; a1 == a8;
  a2 == a1; a2 == a2; a2 == a3; a2 == a4; a2 == a5; a2 == a6; a2 == a7; a2 == a8;
  a3 == a1; a3 == a2; a3 == a3; a3 == a4; a3 == a5; a3 == a6; a3 == a7; a3 == a8;
  a4 == a1; a4 == a2; a4 == a3; a4 == a4; a4 == a5; a4 == a6; a4 == a7; a4 == a8;
  a5 == a1; a5 == a2; a5 == a3; a5 == a4; a5 == a5; a5 == a6; a5 == a7; a5 == a8;
  a6 == a1; a6 == a2; a6 == a3; a6 == a4; a6 == a5; a6 == a6; a6 == a7; a6 == a8;
  a7 == a1; a7 == a2; a7 == a3; a7 == a4; a7 == a5; a7 == a6; a7 == a7; a7 == a8;
  a8 == a1; a8 == a2; a8 == a3; a8 == a4; a8 == a5; a8 == a6; a8 == a7; a8 == a8;
  a1 == a1; a1 == a2; a1 == a3; a1 == a4; a1 == a5; a1 == a6; a1 == a7; a1 == a8;
  a2 == a1; a2 == a2; a2 == a3; a2 == a4; a2 == a5; a2 == a6; a2 == a7; a2 == a8;
  a3 == a1; a3 == a2; a3 == a3; a3 == a4; a3 == a5; a3 == a6; a3 == a7; a3 == a8;
  a4 == a1; a4 == a2; a4 == a3; a4 == a4; a4 == a5; a4 == a6; a4 == a7; a4 == a8;
  a5 == a1; a5 == a2; a5 == a3; a5 == a4; a5 == a5; a5 == a6; a5 == a7; a5 == a8;
  a6 == a1; a6 == a2; a6 == a3; a6 == a4; a6 == a5; a6 == a6; a6 == a7; a6 == a8;
  a7 == a1; a7 == a2; a7 == a3; a7 == a4; a7 == a5; a7 == a6; a7 == a7; a7 == a8;
  a8 == a1; a8 == a2; a8 == a3; a8 == a4; a8 == a5; a8 == a6; a8 == a7; a8 == a8;
  a1 == a1; a1 == a2; a1 == a3; a1 == a4; a1 == a5; a1 == a6; a1 == a7; a1 == a8;
  a2 == a1; a2 == a2; a2 == a3; a2 == a4; a2 == a5; a2 == a6; a2 == a7; a2 == a8;
  a3 == a1; a3 == a2; a3 == a3; a3 == a4; a3 == a5; a3 == a6; a3 == a7; a3 == a8;
  a4 == a1; a4 == a2; a4 == a3; a4 == a4; a4 == a5; a4 == a6; a4 == a7; a4 == a8;
  a5 == a1; a5 == a2; a5 == a3; a5 == a4; a5 == a5; a5 == a6; a5 == a7; a5 == a8;
  a6 == a1; a6 == a2; a6 == a3; a6 == a4; a6 == a5; a6 == a6; a6 == a7; a6 == a8;
  a7 == a1; a7 == a2; a7 == a3; a7 == a4; a7 == a5; a7 == a6; a7 == a7; a7 == a8;
  a8 == a1; a8 == a2; a8 == a3; a8 == a4; a8 == a5; a8 == a6; a8 == a7; a8 == a8;
  a1 == a1; a1 == a2; a1 == a3; a1 == a4; a1 == a5; a1 == a6; a1 == a7; a1 == a8;
  a2 == a1; a2 == a2; a2 == a3; a2 == a4; a2 == a5; a2 == a6; a2 == a7; a2 == a8;
  a3 == a1; a3 == a2; a3 == a3; a3 == a4; a3 == a5; a3 == a6; a3 == a7; a3 == a8;
  a4 == a1; a4 == a2; a4 == a3; a4 == a4; a4 == a5; a4 == a6; a4 == a7; a4 == a8;
  a5 == a1; a5 == a2; a5 == a3; a5 == a4; a5 == a5; a5 == a6; a5 == a7; a5 == a8;
  a6 == a1; a6 == a2; a6 == a3; a6 == a4; a6 == a5; a6 == a6; a6 == a7; a6 == a8;
  a7 == a1; a7 == a2; a7 == a3; a7 == a4; a7 == a5; a7 == a6; a7 == a7; a7 == a8;
  a8 == a1; a8 == a2; a8 == a3; a8 == a4; a8 == a5; a8 == a6; a8 == a7; a8 == a8;
  a1 == a1; a1 == a2; a1 == a3; a1 == a4; a1 == a5; a1 == a6; a1 == a7; a1 == a8;
  a2 == a1; a2 == a2; a2 == a3; a2 == a4; a2 == a5; a2 == a6; a2 == a7; a2 == a8;
  a3 == a1; a3 == a2; a3 == a3; a3 == a4; a3 == a5; a3 == a6; a3 == a7; a3 == a8;
  a4 == a1; a4 == a2; a4 == a3; a4 == a4; a4 == a5; a4 == a6; a4 == a7; a4 == a8;
  a5 == a1; a5 == a2; a5 == a3; a5 == a4; a5 == a5; a5 == a6; a5 == a7; a5 == a8;
  a6 == a1; a6 == a2; a6 == a3; a6 == a4; a6 == a5; a6 == a6; a6 == a7; a6 == a8;
  a7 == a1; a7 == a2; a7 == a3; a7 == a4; a7 == a5; a7 == a6; a7 == a7; a7 == a8;
  a8 == a1; a8 == a2; a8 == a3; a8 == a4; a8 == a5; a8 == a6; a8 == a7; a8 == a8;
  a1 == a1; a1 == a2; a1 == a3; a1 == a4; a1 == a5; a1 == a6; a1 == a7; a1 == a8;
  a2 == a1; a2 == a2; a2 == a3; a2 == a4; a2 == a5; a2 == a6; a2 == a7; a2 == a8;
  a3 == a1; a3 == a2; a3 == a3; a3 == a4; a3 == a5; a3 == a6; a3 == a7; a3 == a8;
  a4 == a1; a4 == a2; a4 == a3; a4 == a4; a4 == a5; a4 == a6; a4 == a7; a4 == a8;
  a5 == a1; a5 == a2; a5 == a3; a5 == a4; a5 == a5; a5 == a6; a5 == a7; a5 == a8;
  a6 == a1; a6 == a2; a6 == a3; a6 == a4; a6 == a5; a6 == a6; a6 == a7; a6 == a8;
  a7 == a1; a7 == a2; a7 == a3; a7 == a4; a7 == a5; a7 == a6; a7 == a7; a7 == a8;
  a8 == a1; a8 == a2; a8 == a3; a8 == a4; a8 == a5; a8 == a6; a8 == a7; a8 == a8;
  a1 == a1; a1 == a2; a1 == a3; a1 == a4; a1 == a5; a1 == a6; a1 == a7; a1 == a8;
  a2 == a1; a2 == a2; a2 == a3; a2 == a4; a2 == a5; a2 == a6; a2 == a7; a2 == a8;
  a3 == a1; a3 == a2; a3 == a3; a3 == a4; a3 == a5; a3 == a6; a3 == a7; a3 == a8;
  a4 == a1; a4 == a2; a4 == a3; a4 == a4; a4 == a5; a4 == a6; a4 == a7; a4 == a8;
  a5 == a1; a5 == a2; a5 == a3; a5 == a4; a5 == a5; a5 == a6; a5 == a7; a5 == a8;
  a6 == a1; a6 == a2; a6 == a3; a6 == a4; a6 == a5; a6 == a6; a6 == a7; a6 == a8;
  a7 == a1; a7 == a2; a7 == a3; a7 == a4; a7 == a5; a7 == a6; a7 == a7; a7 == a8;
  a8 == a1; a8 == a2; a8 == a3; a8 == a4; a8 == a5; a8 == a6; a8 == a7; a8 == a8;
}
var elapsed = clock() - start;
print \"elapsed: \";
print elapsed - loopTime;";
pub const FIB_LOX: &str = "fun fib(n) {
  if (n < 2) return n;
  return fib(n - 2) + fib(n - 1);
}
var start = clock();
fib(35);
print \"elapsed: \";
print clock() - start;";
