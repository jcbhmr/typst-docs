---
description: |
  您是 LaTeX 用户吗？本指南解释了 Typst 和 LaTeX 之间的差异和相似之处，以便您可以快速入门。
---

# LaTeX 用户指南
这篇文章是写给那些已经有过 LaTeX 使用经验，并想要尝试 Typst 的用户。
我们将从用户的角度出发，解释这两套系统之间主要的不同。
虽然 Typst 并不是基于 LaTeX 构建的，语法也完全不同，但通过这篇文章，你可以方便地将 LaTeX 的使用经验转化过来。

<!-- Mention that Typst is not built upon LaTeX -->

跟 LaTeX 一样，Typst 是一门基于标记的排版系统。首先在纯文本里编写自己的文档，然后通过一些命令和语法对其进行修饰，最后使用编译器将源文件排版并渲染成 PDF 文件。在以下几个方面，Typst 跟 LaTeX 之间也存在区别：日常任务上，Typst 使用更为专用的语法，这一点可以类比 Markdown。Typst 的命令也更有「原则性」：在 LaTeX 中，你可能需要为不同软件包学习不同的语法和约定，但在 Typst 中，命令总是保持一致，所以你只需要理解一些基本的概念。并且 Typst 比 LaTeX 快得多：Typst 文件的编译通常只需要几毫秒，而不是几秒，因此 Typst 可以实现实时增量渲染。

在接下来的部分，我们将回答从 LaTeX 切换到 Typst 时一些常见的问题。
如果你更喜欢一步一步地介绍 Typst，请阅读我们的 [教程]($tutorial)。

## 如何创建一个新的空文档？{ #getting-started }
很简单，你只需要创建一个空的文本文档（文件后缀是.typ），无需额外的模板，你就可以直接开始编写。
如果您使用的是 Web App，请单击“+ Empty document”以创建一个带有文件的新项目，然后进入编辑器。

[段落分隔符]($parbreak) 和 LaTeX 相同，空一行即可：

```example
Hey there!

Here are two paragraphs. The
output is shown to the right.
```

## 我如何创建章节标题，强调，...？{ #elements }
LaTeX 使用 `\section` 命令创建章节标题。多级标题分别用 `\subsection`、`\subsection` 等表示。根据文档种类的不同，还有 `\part` 和 `\chapter`。

在 Typst 中，标题设置更简洁：在标题所在的行前面加上一个等号和一个空格，便得到了一级标题：`[= Introduction]`。
如果你需要一个二级标题，则可以使用两个等号：`[== In this paper]`。
通过在前面加上更多的等号，你可以嵌套任意层级的标题。

<div class="info-box">
译者注：
这一点上更接近 Markdown 中 `#` 的作用，在接下来的阅读中你会不断看到这种「Markdown + LaTeX」杂糅的产物，结合这两者分别的痛点，可以更加深入了解 Typst 设计这些语法的原因。
</div>

强调（通常以斜体字呈现）是通过用`[_underscores_]`来表达，
而着重的强调（通常以黑体字呈现）是通过使用`[*Star*]`来代替。

强调（英文中以斜体呈现）只需要把文本包含在 `_下划线_` 中，
粗体则需要包含在 `*单个星号*` 之中。

下面是 LaTeX 中使用的常见标记命令及其 Typst 对应的表示方式。你也可以查看[完整的语法备忘单]($syntax)。

| 元素              | LaTeX                     | Typst                  | See                      |
|:-----------------|:--------------------------|:-----------------------|:-------------------------|
| 着重强调          | `\textbf{strong}`         | `[*strong*]`           | [`strong`]($strong) |
| 强调             | `\emph{emphasis}`         | `[_emphasis_]`         | [`emph`]($emph)     |
| 等宽文字 / 代码    | `\texttt{print(1)}`       | ``[`print(1)`]``       | [`raw`]($raw)       |
| 链接             | `\url{https://typst.app}` | `[https://typst.app/]` | [`link`]($link)     |
| 标签             | `\label{intro}`           | `[<intro>]`            | [`label`]($label)   |
| 交叉引用          | `\ref{intro}`             | `[@intro]`             | [`ref`]($ref)       |
| 文献引用          | `\cite{humphrey97}`       | `[@humphrey97]`        | [`cite`]($cite)     |
| 无序列表          | `itemize` 环境            | `[- List]`             | [`list`]($list)     |
| 有序列表          | `enumerate` 环境          | `[+ List]`             | [`enum`]($enum)     |
| 术语列表          | `description` 环境        | `[/ Term: List]`       | [`terms`]($terms)   |
| 图片             | `figure` 环境             | `figure` 函数           | [`figure`]($figure) |
| 表格             | `table` 环境              | `table` 函数            | [`table`]($table)   |
| 公式             | `$x$`, `align` / `equation` 环境 | `[$x$]`, `[$ x = y $]` | [`equation`]($math.equation) |

在 Typst 中，使用 [列表]($list) 并不需要「环境」，而采用一种更为轻量的语法。
只需要在每行开头前，加入连字符 `-`，就可以创建一个无序列表：

````example
To write this list in Typst...

```latex
\begin{itemize}
  \item Fast
  \item Flexible
  \item Intuitive
\end{itemize}
```

...just type this:

- Fast
- Flexible
- Intuitive

````

使用正确的缩进，可以实现嵌套列表。且在每项间添加空行，可以得到一个行间距更大的列表

要获得一个 [有序列表]($enum)（ `enumerate` ），可以用 `+` 代替连字符。
对于一个 [术语列表]($terms)（ `description` ），可以用 `[/ Term: Description]` 来表示。

## 我如何使用一个命令？ { #commands }
LaTeX 十分依赖以反斜杠 `\` 开头的命令，它需要通过这些 _宏_ 来排版、插入或改变内容。
有些命令可以传入参数，这些参数通常使用大括号括起来。`\cite{rasmus}`

Typst 区分两种模式：[「标记模式」和「脚本模式」]($scripting/#blocks)。
默认是「标记模式」。此模式下，你可以直接编排文本、使用不同的语法结构，如 `*使用星号标记粗体文本* `。
而「代码模式」下，则更类似像 Python 一样的编程语言，提供了输入、执行代码的选项。

在 Typst 的标记模式中，你可以使用井号（`#`）来使用单个命令（或者表达式）。
例如，你可以通过这种方式来分割不同的 [文件]($scripting/#modules)，或者基于某些 [条件]($scripting/#conditionals) 渲染文字。在「脚本模式」下，也可以通过方括号来包含正常的（「标记模式」下的）[内容块]($content)。
在「脚本模式」下，内容（标记模式的整体）会被跟其他变量一样，当作一个值本身进行处理。

```example
First, a rectangle:
#rect()

Let me show how to do
#underline([_underlined_ text])

We can also do some maths:
#calc.max(3, 2 * 4)

And finally a little loop:
#for x in range(3) [
  Hi #x.
]
```

<div class="info-box">
译者注：这段英文原文就表述的很不清晰，这里提供一点解释。

标记模式，如同 Markdown，不需要额外的内容，默认就处在这个标记模式下：可以直接使用 `_text_` 或者 `*text*` 来实现斜体/粗体（参考上文）

脚本模式，以 `#` 开头（类比 LaTeX 中以 `\` 开头来书写命令），仅仅存在于一个命令（表达式）中，当这个表达式结束之后，一切又回到标记模式中。
在脚本模式中，无需额外使用 `#`（命令/关键字会直接识别，不同于 LaTeX），同时在命令中也可以使用标记模式（使用 `[]`，比如例子给出的 `[Hi #x]`）

另外：

- 在标记模式中，可以引用脚本模式的函数、值，都通过 `#` 进行标记
- 在脚本模式中，函数返回值为 `content` 的会在当前位置渲染出来如果没有显示指明返回值，则默认会将函数体内的所有内容块相加并返回
- 在脚本模式中，使用标记模式的内容 (Content)，也可以作为变量的值
</div>

一个函数调用总是涉及到函数的名称（[`rect`]($rect), [`underline`]($underline), 
[`calc.max`]($calc.max), [`range`]($array.range)），后面是小括号（与 LaTeX 不同，如果宏不需要参数，方括号和大括号是可选的）。
在这些圆括号内传递的参数的预期列表取决于具体的函数，并在 [引用]($reference) 中指定。

### 参数 { #arguments }
一个函数可以有多个传入参数，有些参数是位置参数，只需提供变量的值即可（不需要提供参数名）：函数 `[#lower("SCREAM")]` 以全小写的方式返回其传入值。
很多函数使用命名参数来提高可读性，例如创建一个指定大小和描边的正方形，可以使用如下命名参数：

```example
#rect(
  width: 2cm,
  height: 1cm,
  stroke: red,
)
```

你指定一个命名参数，首先输入它的名字（上文中是它`width`、`height`和`stroke`），然后是冒号，接着是值（`2cm`、`1cm`、`red`）。
你可以在每个函数的参考页中找到可用的命名参数，或者在输入时的自动完成面板中找到。
命名参数类似于一些 LaTeX 环境的配置方式，例如，你可以输入`\begin{enumerate}[label={alph*)}]`来启动一个带有标签`a)`、`b)`等的列表。

指定命名参数时，你需要首先输入参数的名字，例如 `width`、`height` 和 `stroke` 等，接下来输入冒号和对应的值 (`2cm`、`1cm`、`red`) ，你可以在文档或者自动补全中找到可以传入的参数。
命名参数有些类似 LaTeX 中配置环境的办法，比如 `\begin{enumerate}[label={alph*)}]` 可以作为一个带有标签`a)`、`b)`等的列表的起始标志。

大多数情况下，你会希望向函数传递一些 [内容块]($content)。
例如，在 LaTeX 中的命令 `\underline{Alternative A}`
在 Typst 中则可以写成 `#underline([Alternative A])`，方括号表示其中的值是一个内容块 。
在这些方括号中，你可以使用正常的标记语法。然而，都这样写的话，需要嵌套的括号数量还是太多了，因此你可以把位于尾部的内容块放到括号后面（如果没有其他参数，甚至可以忽略 `()`）：

```example
Typst is an #underline[alternative]
to LaTeX.

#rect(fill: aqua)[Get started here!]
```

### 数据类型 { #data-types }
你很可能已经注意到了，上文提到的这些参数有着不同的数据类型。Typst支持多种 [数据类型]($type)，下表是其中一些比较重要的类型和以及声明他们的办法。

注意：为了使用以下任何类型，你必须处在脚本模式中。

| 数据类型                            | 示例                           |
|:-------------------------------------|:----------------------------------|
| [内容块 (content)]($content)             | `{[*fast* typesetting]}`          |
| [字符串 (str)]($str)               | `{"Pietro S. Author"}`            |
| [整型 (int)]($int)             | `{23}`                            |
| [浮点数 (float)]($float) | `{1.459}`                         |
| [绝对长度 (absolute length)]($length)      | `{12pt}`, `{5in}`, `{0.3cm}`, ... |
| [相对长度 (relative length)]($ratio)       | `{65%}`                           |

内容块和字符串的区别在于，内容可以包含标记，包括函数调用，而字符串实际上只是一个普通的字符序列。

Typst 提供了 [条件分支、循环结构]($scripting/#conditionals)，以及常用的运算符，比如 `+` 和 `==`，你也可以定义你自己的变量，并在此基础上进行计算。

### 影响后续内容的命令 { #rules }
在 LaTeX 中，有些命令，例如 `\textbf{bold text}` 通过大括号传入参数，并且只影响括号内的内容。而有些命令，比如 `\bfseries bold text`「起到开关的作用」，在这行命令后的所有内容都会受这个命令的影响。

在 Typst 中，同样的函数可以同时用来影响：

- 文档的剩余部分
- 一段区域（作用域）
- 它的参数

举例来说，`[#text(weight: "bold")[bold text]]` 仅仅会加粗传入的参数，而 `#set text(weight: "bold")` 会影响「到作用域结束」，（或者，如果不在作用域中，影响文档的剩余部分）。根据使用方式的不同（函数调用 / 使用 Set 规则），很容易区分它的作用域。

```example
I am starting out with small text.

#set text(14pt)

This is a bit #text(18pt)[larger,]
don't you think?
```

Set 规则可能出现在文档的任何部分。它们可以被认为是其各自函数的默认参数值：

```example
#set enum(numbering: "I.")

Good results can only be obtained by
+ following best practices
+ being aware of current results
  of other researchers
+ checking the data for biases
```

`+` 是调用 [`{enum}`]($enum) 函数的语法糖（可以把它看作是一种简写），我们在上面应用了一个 Set 规则。
从这个意义上讲，[大多数的特殊语法都是只是某一个函数的简写]($syntax)。

如果你需要重新定义一个组件的样式（超过了修改其参数所能提供的），你可以通过 [Show 规则]($styling/#show-rules) 完全重定义其样式（与 LaTeX 中 `\renewcommand` 相似，类似于定义了一个宏）.

## 如何加载一个文档类 / 模板？ { #templates }
在 LaTeX 中，你可以使用 `\documentclass{article}` 来定义文档的样式。通过这个命令，你也可以通过把 `article` 更换为 `report`和 `amsart` 来更换文档的样式。

在使用 Typst 中，你可以通过 [函数]($function) 来修改文档的样式。通常情况下，你可以使用模板中提供的函数来修改整个文档，使用方式如下：你可以通过 `#import` 来导入模板函数，然后你使用这个函数来对文档使用样式。具体的做法是通过 [Show 规则]($styling/#show-rules) 来将整个文档包装在这个函数中，具体如下：

```example:single
>>> #let conf(
>>>   title: none,
>>>   authors: (),
>>>   abstract: [],
>>>   doc,
>>> ) = {
>>>  set text(font: "Linux Libertine", 11pt)
>>>  set par(justify: true)
>>>  set page(
>>>    "us-letter",
>>>    margin: auto,
>>>    header: align(
>>>      right + horizon,
>>>      title
>>>    ),
>>>    numbering: "1",
>>>  )
>>>
>>>  show heading.where(
>>>    level: 1
>>>  ): it => block(
>>>    align(center,
>>>      text(
>>>        13pt,
>>>        weight: "regular",
>>>        smallcaps(it.body),
>>>      )
>>>    ),
>>>  )
>>>  show heading.where(
>>>    level: 2
>>>  ): it => box(
>>>    text(
>>>      11pt,
>>>      weight: "regular",
>>>      style: "italic",
>>>      it.body + [.],
>>>    )
>>>  )
>>>
>>>  set align(center)
>>>  text(17pt, title)
>>>
>>>  let count = calc.min(authors.len(), 3)
>>>  grid(
>>>    columns: (1fr,) * count,
>>>    row-gutter: 24pt,
>>>    ..authors.map(author => [
>>>      #author.name \
>>>      #author.affiliation \
>>>      #link("mailto:" + author.email)
>>>    ]),
>>>  )
>>>
>>>  par(justify: false)[
>>>    *Abstract* \
>>>    #abstract
>>>  ]
>>>
>>>  set align(left)
>>>  columns(2, doc)
>>>}
<<< #import "conf.typ": conf
#show: conf.with(
  title: [
    Towards Improved Modelling
  ],
  authors: (
    (
      name: "Theresa Tungsten",
      affiliation: "Artos Institute",
      email: "tung@artos.edu",
    ),
    (
      name: "Eugene Deklan",
      affiliation: "Honduras State",
      email: "e.deklan@hstate.hn",
    ),
  ),
  abstract: lorem(80),
)

Let's get started writing this
article by putting insightful
paragraphs right here!
```

这里的 [`import`]($scripting/#modules) 命令，导入了在其他文件中声明的函数。在这个例子中，它从 `conf.typ` 中导入了 `conf` 函数。这个函数会将整个文档展示为一个会议论文。我们通过 Show 规则讲这个样式应用到全局，同时设置了文档的一些元数据。在应用 Show 规则之后，我们就可以开始写文章了。

<div class="info-box">

在 Typst 中，函数被称为"命令"，它们可以将其参数转化为输出值，包括文档 _内容_。
函数是"纯净"的，这意味着它们除了创建一个输出值/输出内容外，不能产生任何其他效果。
这与 LaTeX 的宏形成了鲜明的对比，后者可以对你的文档产生任意的效果。

函数算作 Typst 的一种命令，而且可以将输入的参数转换为所需要的输出，包括内容块。在 Typst 中函数都是「过程的、纯的」，意味着他们除了返回值外不能有任何其他的副作用。
这与 LaTeX 的宏形成了鲜明的对比，后者可以对你的文档产生任意的效果。

要让一个函数为你的整个文档提供样式，显示规则会处理它后面的所有内容，并以结果为参数调用冒号后面指定的函数。
`.with` 部分是一个方法，它接收 `conf` 函数，并在传递给显示规则之前预先配置它的一些参数。

为了将一个样式应用到整个文档，Show 规则会把后续的所有内容都作为参数，传递给冒号后面定义的函数。`.with` 是函数对象的一个方法，它可以预设一些其他的默认参数（类似于柯里化），在传递给 Show 规则之前进行一些设置更改。

译者注：`#show: conf.with(title: [标题])` 等价于 Lambda 表达式形式的 `#show: it => conf(title: [标题], it)`
</div>

在 Web App 中，你可以选择一些预先定义好的模板，甚至可以通过模板向导创建自己的模板。
你也可以访问 [Awesome Typst 仓库](https://github.com/qjcg/awesome-typst) 来查看一些社区提供的模板。我们正在筹备为包管理器增加托管模板的能力（目前只能托管工具类包），在那之后这一切都会更加容易分享！

你也可以 [创建你自己的自定义的模板]($tutorial/making-a-template)。它们比相应的 LaTeX 的 `.sty` 文件短得多，可读性也高得多，所以不妨一试!

## 如何导入包？ { #packages }
Typst 是 "即插即用" 的，所以许多流行的 LaTeX 包的对应功能是直接内置到 Typst 里的。
在下面我们列出一些 LaTeX 中常用的包，和他们对应的 Typst 命令：

| LaTeX 包                        | Typst 替代                                                            |
|:--------------------------------|:---------------------------------------------------------------------|
| graphicx, svg                   | [`image`]($image) 函数                                           |
| tabularx                        | [`table`]($table), [`grid`]($grid) 函数                     |
| fontenc, inputenc, unicode-math | 直接编写!                                                              |
| babel, polyglossia              | [`text`]($text.lang) 函数： `[#set text(lang: "zh")]`            |
| amsmath                         | [数学模式]($category/math)                                            |
| amsfonts, amssymb               | [`sym`]($category/symbols) 模块和 [syntax]($syntax/#math)             |
| geometry, fancyhdr              | [`page`]($page) 函数                                            |
| xcolor                          | [`text`]($text.fill) 函数： `[#set text(fill: rgb("#0178A4"))]`  |
| hyperref                        | [`link`]($link) 函数                                            |
| bibtex, biblatex, natbib        | [`cite`]($cite), [`bibliography`]($bibliography) 函数      |
| lstlisting, minted              | [`raw`]($raw) 函数和语法                                         |
| parskip                         | [`block`]($block.spacing) 和 [`par`]($par.first-line-indent) 函数 |
| csquotes                        | 设置 [`text`]($text.lang) 语言，并输入 `["]` or `[']`             |
| caption                         | [`figure`]($figure) 函数                                        |
| enumitem                        | [`list`]($list), [`enum`]($enum), [`terms`]($terms) 函数 |


尽管 _很多_ 东西是内置的，但并非所有东西都可以内置。这就是为什么 Typst 有一个内置的 [包管理器]($packages)，社区可以在其中共享他们的工作和自动化工具。让我们以 _tablex_ 包为例：此包允许您以内置的表格尚不支持的方式自定义表格。要在文档中使用 tablex，您只需编写：

```typ
#import "@preview/tablex:0.0.6": tablex, gridx
```

(`@preview` 是一个 _namespace_，在包管理器还处于早期和实验状态时使用，它将在将来被替换。)

除了官方的软件包存储库，您可能还会想看
[Awesome Typst 仓库](https://github.com/qjcg/awesome-typst)，其中集合了为 Typst 创建的资源精选列表。

译者注：以及 [Awesome Typst 中文仓库](https://github.com/typst-doc-cn/awesome-typst-cn)。

如果您需要从项目中的另一个文档加载函数和变量，例如使用模板，则可以使用相同的
[`{import}`]($scripting/#modules) 语句，其中应该包含文档名，而不是特定的包。要包含另一个文档的文本内容,
您可以使用 [`{include}`]($scripting/#modules) 语句。它将读取指定文档的内容，并将其直接置入文档中。

## 如何输入数学公式？ { #maths }
要在 Typst 中进入数学模式，只需将数学公式用 `$` 符号括起来。你可以通过在方程内容和其周围的 `$` 符号之间添加空格或换行来书写块状公式（行间公式）。

```example
The sum of the numbers from
$1$ to $n$ is:

$ sum_(k=1)^n k = (n(n+1))/2 $
```

[数学模式]($category/math) 的工作方式与普通标记或代码模式不同。数字和单个字符被逐字显示，而多个连续（非数字）字符将被解释为 Typst 变量。

Typst 在数学模式下预先定义了很多有用的变量。所有希腊字母（`alpha`, `beta`, ...）和一些希伯来字母（`alef`, `bet`, ...）都可以通过它们的名字直接使用。
一些符号还可以通过缩写轻松使用，如 `<=`、`>=` 和 `->`。

符号的完整列表请参考 [符号页面]($symbol)。如果缺少某些符号，你也可以通过 [Unicode 转义序列]($syntax/#escapes) 访问它。

符号的变体通常可以通过在句点后附加一个 [`.` 点修饰符]($symbol) 来选择。例如，`arrow.l.squiggly` 插入了一个向左倾斜的箭头。
如果你想在你的表达式中插入多字母纯文本，可以用双引号将其括起来：

```example
$ delta "if" x <= 5 $
```

在 Typst 中，定界符将根据内部表达式自动缩放大小，就像在 LaTeX 中自动添加了隐藏的 `\left` 和 `\right` 命令一样。
你可以使用 [`lr`]($math.lr) 函数自定义定界符的行为。如果你不需要对定界符进行缩放，你可以用反斜线转义定界符。

在不破坏运算优先级的前提下，Typst 会自动将斜线 `/` 的两端内容识别成分数。所有没必要的括号将不会出现在编译结果中：

```example
$ f(x) = (x + 1) / x $
```

[下标和上标]($math.attach) 在 Typst 和 LaTeX 中的作用是相似的。`{$x^2$}` 将产生一个上标，`{$x_2$}` 产生一个下标。
如果你想在下标或上标中包含一个以上的值，请把它们的内容放在括号里：`$x_(a -> epsilon)$`。

由于数学模式下的变量不需要在前面加上 `#` 或 `/` ，所以你也可以无需额外的井号字符来调用函数：

```example
$ f(x, y) := cases(
  1 "if" (x dot y)/2 <= 0,
  2 "if" x "is even",
  3 "if" x in NN,
  4 "else",
) $
```

上面的例子用 [`cases`]($math.cases) 函数来表述 `f`。在 `cases` 函数中，参数用逗号来分隔，参数也被解释为数学模式下的内容。
如果你需要传递 Typst 变量，可以用 `#` 号作为前缀使用：

```example
$ (a + b)^2
  = a^2
  + text(fill: #maroon, 2 a b)
  + b^2 $
```

在数学模式下，你可以使用任意的 Typst 函数或者任何内容，如果你希望他们正常工作，只需要使用 `#` 前缀，没人可以阻止你把长方体或者 emoji 表情作为参数传入：

```example
$ sum^10_(🥸=1)
  #rect(width: 4mm, height: 2mm)/🥸
  = 🧠 maltese $
```

如果你希望直接以 Unicode 形式输入数学符号，也是可以的。

数学调用可以有二维参数列表，使用 `;` 作为分隔符。这方面最常见的用途是使用 [`mat`]($math.mat) 函数创建矩阵：

```example
$ mat(
  1, 2, ..., 10;
  2, 2, ..., 10;
  dots.v, dots.v, dots.down, dots.v;
  10, 10, ..., 10;
) $
```

## 如何获得 "LaTeX 外观"？ { #latex-look }
用 LaTeX 编写的论文有一种美观且易于识别的外观。这主要是由于它们的字体 [Computer Modern](https://zh.wikipedia.org/wiki/Computer_Modern)、对齐方式、窄行距和宽边距。

下面是一个示例：
- 设置宽 [边距]($page.margin)
- 启用 [两端对齐]($par.justify), [更紧密的行间距]($par.leading)
  和 [首行缩进]($par.first-line-indent)
- 设置 [字体]($text.font) 为 "New Computer Modern"，这是一个适用于文本和 [代码块]($raw) 的 OpenType 变体
- 禁用段落 [间距]($block.spacing)
- 增加 [标题]($heading) 周围的 [间距]($block.spacing)

```typ
#set page(margin: 1.75in)
#set par(leading: 0.55em, first-line-indent: 1.8em, justify: true)
#set text(font: "New Computer Modern")
#show raw: set text(font: "New Computer Modern Mono")
#show par: set block(spacing: 0.55em)
#show heading: set block(above: 1.4em, below: 1em)
```

这应该是一个很好的起点! 如果你想更进一步，为什么不创建一个可重复使用的模板呢？

## 与 LaTeX 相比，Typst 目前有哪些不足？ { #limitations }
尽管现在 Typst 已经可以成为许多人的 LaTeX 替代品，但仍有一些功能是 Typst （尚）不支持的。
这里列出了相应的不足，在一些情况下，也包含了可能的变通方法。

- **本地图表和绘图。** LaTeX 用户经常在 PGF/TikZ 中与他们的文档一起创建图表。
  Typst 还不包括绘制图表的工具，但社区正在加紧提供解决方案，如
  [`cetz`](https://github.com/johannes-wolf/typst-canvas)。你可以把这些工具添加到你的文档中，开始画图。

- **在没有分页符的情况下改变页边距。** LaTeX 中，你可以在不换页的前提下，调整页边距。
  要在 Typst 中改变页边距，你要使用 [`page`]($page) 函数（会强制换页）。
  如果你只想让几个段落伸进页边距，然后再恢复到旧的页边距，你可以使用带负数填充的 [`pad`]($pad) 函数。

- **将 PDF 作为图像。** 在 LaTeX 中，插入 PDF 或 EPS 文件的矢量图已经成为一种习惯。
  Typst 不支持这两种格式作为图像格式，但你可以用 [在线工具](https://cloudconvert.com/pdf-to-svg)
  或 [Inkscape](https://inkscape.org/) 轻松地将这两种文件转换成 SVG 文件。我们计划在 Typst 的 Web 应用程序中也加入这些文件格式的自动转换功能。

- **分页符优化。** LaTeX 的算法会智能优化换行符和换页符。Typst 虽然也会避免「孤儿」和「寡妇」的出现，但它所使用的算法远远不及 LaTeX 所使用的复杂。
  你可以在提交文档前使用 `[#pagebreak(weak: true)]` 在 Typst 中插入自定义分页。
  参数 `weak` 会保证：如果这里本来就该是很自然的换页，则不会插入两个换页符。
  你也可以使用 `[#v(1fr)]` 在页面末尾插入一些空白，它的工作原理与 LaTeX 的 `\vfill` 相当类似。
  