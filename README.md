# 这里是由xphost制作的Minecraft启动器类库

## 项目名称：

- 该项目立项于2023.4.29，所作之事皆在为简化Minecraft启动器开发做准备。
- 该项目名称原定（2023.3.18）为MoreMineCraftLauncherLib，后为简化名称因此被称为：MMCLL。
- MMCLL：全称正如上面项目名称一致。
- 该项目致力于用任意一款编程语言开发Minecraft Launcher做准备。

## 这里分为几类类库：

1. C++类库
2. Java类库
3. C#类库
4. Python类库
5. Delphi类库
6. 易语言类库
7. Go语言类库
8. Rust类库
9. TypeScript类库

- 这几种类库为作者随意制作，可能有些许的不足，但这些事也几乎微不足道。如果你还有更好的建议，欢迎在此处提出issue。

## 使用方法：

1. 进入你所需要的源码库，源码库类型见下方：

|语言|源码库文件夹|完成进度|
|----|----|----|
|C++|CppVersion|0%|
|Java|JavaVersion|0%|
|C#|CSVersion|0%|
|Python|PythonVersion|3%|
|Delphi|DelphiVersion|8%|
|易语言|EVersion|0%|
|Go|GoVersion|0%|
|Rust|RustVersion|15%|
|TypeScript|TSVersion|0%|

- 其中，很多种文件夹里面都是只有一个文件的，只有极少数文件夹里面有多个文件。例如：

1. Delphi里面可能会有dproj、dpr文件、
2. Rust里面可能会有Cargo.toml文件等。
3. Java里面可能会有maven.xml文件

- 其中，由于易语言的库是直接发布【.e】后缀的源码，而非ec或者dll之类的类库，因此大家需要在易语言里导入这个e文件，然后在e文件里自行复制源码之后再导入自己的工程就好了。

## 许可协议：

- 本类库使用的协议为MIT协议，大家可以随意分发，也同意大家制作出的启动器进行商业闭源。但是你必须遵守开源协议，在制作出的启动器中对该库作者进行鸣谢。
- 该库与Little Limbo Launcher的开源库许可证不相同，LLL启动器使用的开源协议是GPL。而这个类库使用的协议是MIT。

## 项目规范：

- 文件夹内还贴心的配备了README.md文件，用于指导大家下载所需要的依赖。【如C#需要的Newtonsoft.JSON、Java所需要的fastjson等。】
- 大家只需要按需下载即可。如果遇到只有单个文件内容的源码，那你几乎可以不下载源文件，直接将源码复制粘贴到你的工作空间即可。然后按照README.md文件中，配好环境依赖即可。
- 本类库中，无论哪一种语言，我都贴心的为所有的方法或者是函数添加了注释。因此不必担心无法理解函数的内容。【虽然函数内容并未做注释，但是函数的开头我都做了的！】
- 同样的，在每一个文件夹中，我都贴心的为该语言单独设立了一个SPECIFIC.md，用于为该类中的每一个函数做一个详细的解读。
- 我在每个文件夹中，提供了一个README.md，包括但不限于【目前实现的功能、版本号、工作进度等】。因此，文件夹里的README.md算是对这门语言最详细的教导了。
- 我甚至还在每个文件夹里提供了一个EXAMPLE.md，用于昭告这个语言使用示例，如如何启动游戏等。

## 项目使用IDE工具（以及建议使用开发工具）

|语言|IDE工具|
|----|----|
|C++|Qt Creator|
|Java|Jetbrains Intellij IDEA|
|C#|Jetbrains Rider|
|Python|PyScripter|
|Delphi|RAD Studio|
|易语言|易语言|
|Go|Jetbrains GoLand|
|Rust|Jetbrains RustRover|
|TypeScript|Jetbrains WebStorm|

- 别问我为什么这么喜欢JetBrains家的玩意，因为爱情！

## 问题？

- 如果你有任何问题，欢迎在此github中提出问题。
- 如果你自己习惯使用的语言在本类库中并没有，你可以提一个issue，或者如果你想贡献代码，也可以提交pull-request！
- 你当然可以为我已经弃坑的语言提一个pull-request，但是请别提出issue了。
- 本类库不会再提供的语言有：

|不会提供的编程语言|原因|弃坑|
|----|----|----|
|C|作为C++的前身，已经有C++了。|是|
|Swift|作为IOS独特的语言，我不会（没有mac……|是|
|Ruby|可以考虑一下，如果有人为我提出issue的话|待考虑|
|Perl|啥都没有，我都不会这门语言|是|
|Lua|也是一样的，啥也没有|是|
|Julia|更一样了，啥也没有|是|
|Zig、Pony、Pike、Vala|这些等异常冷门的语言，我不会|是|

## 关于贡献名单

1. 如果你为作者提交了一个issue，并且提交的是已有语言的bug，那我可能会将你添加至：【优秀贡献人员】中。
2. 如果你为作者提交了一个issue，并且在已有语言的基础上，详细说明添加某个该有的函数或功能，那我可能会将你加入【精品贡献人员】中。
3. 如果你为作者提交了一个pull request，直接帮助作者修复已有源码的bug，那我可能会将你加入【史诗贡献人员】中。
4. 如果你为作者提交了一个pull request，并且这个pull request是另一门非常火的语言，你帮作者完善了类库，那我可能会将你加入【传奇贡献人员】中。

提交issue报告流程：
```
1. 标题：【bug汇报、建议】+名称

2. 你要汇报的内容是什么？

3. 你觉得这个内容会对开发者产生什么影响？

【以下内容仅针对bug提交】

4. 该【bug】出现在【源代码函数名】？

5. 你是否针对此有了更好的解决方案？【如果你有更好的解决方案，你应该提交pull request而不是issue！】

6. 你对该函数的输入参数是【】，输出的值是【】，你的预期结果是什么？

【以下内容仅针对建议】

4. 该功能是否能解决开发者的部分需求？【此项需填写你的需求】

5. 如果作者解决了，你是否愿意无条件继续使用该模块？

【以下针对所有】

7. 你要提交的备注
```

对于提交pull request的话，没那么多讲究，作者只会看看你提交的源代码是否符合规范！