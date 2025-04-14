# PackPal

静态Markdown博客站点生成工具，将markdown文件打包成静态博客站点。

## 使用指南

### 新建项目

`packpal new [project name]`：在当前文件夹下生成一个新的项目文件，默认的项目文件包括：

```txt
根目录(你的项目名字)
├── posts/                  	(Markdown源文件存放的地方)
│   ├── metadata.json         	(存放文章的元数据，如标题、日期等，还在这里配置博客的基本信息)
│   └── 我的第一篇博客.md        	(示例博客文章)
├── templates/               	(模板HTML存放地方)
│   ├── posts_template.html  	(文章模板文件)
│   └── index_template.html   	(主页模板文件)
├── build/                    	(用来存放打包合成后的文件)
├── config.json					(默认的配置文件)
└── README.md                 	(PackPal使用指南)
```

配置文件`config.json`内包含站点的配置文件，当第一次使用packpal的时候，用户需要配置这个文件，控制站点生成的内容。`config.json`默认配置内容如下：

```json
{
  "username": "PackPal",
  "avatar": "./avatar.jpg",
  "contact": {
    "github": "",
    "phone": "",
    "email": ""
  },
  "templates_dir": "./templates/",
  "posts_dir": "./posts/",
  "build_dir": "./build/",
  "plugins": []
}
```



### 生成博客

`packpal build`：将markdown文件翻译成html字符串后，和HTML模板拼接，生成静态文件。

模板文件包含页面内的文章的默认样式和一些默认组件（例如博客导航栏和文章目录）。模板文件内包含特殊代码，如`<ContentRoot/>`、`<PostHeading/>`。这些特殊的代码在生成阶段将被替换成由markdown翻译而来的HTML字符串，最后生成静态文件。

```txt
[模板文件 templates.html] --------------------------------─┐
[markdown文件]-------(翻译)--------> [html字符串]--------(字符串拼接)----------> 静态站点文件
```

最后生成的文件会包含一个`index.html`文件以及生成的post文件。

```txt
build/
├── public/
│	├── avatar.png
│	└── ...其他图片或者静态资源
├── articles/
│	├── 文章1.html
│	└── ...其他文章
└── index.html
```

每次运行build指令，packpal都会在当前目录下查找`config.json`文件，尝试读取其中的内容。如果当前运行目录下找不到`config.json`文件，packpal会创建一个默认的配置文件，并且使用其中的配置。

### 部署博客

`packpal deploy`：自动将`build/`文件夹下生成的静态文件推送到远程github pages仓库。

### 更新博客

`packpal update`：自动更新博客内容，即自动连续执行 `packpal clean` 、`packpal build`、 `packpal deploy`三条指令 。

更新博客内容需要将新的markdown文件放到`./posts/`文件夹下，并且更新posts文件夹下`metadata.json`中的内容，最后使用update指令，将更新运用到远程服务器。

### 删除生成内容

`packpal clean` ：这条命令将自动删除`./build/`目录下所有生成的内容。



## 插件 Plugins
