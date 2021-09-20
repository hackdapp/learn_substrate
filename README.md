# learn_substrate（第五课作业ERC20）

## test

```
> cargo +nightly test
```

![](http://cdn.hackdapp.com/2021-09-20-012608.png)

## build

```
> cargo +nightly contract build
```

![](http://cdn.hackdapp.com/2021-09-20-015728.png)

## Run Local Node

### install node

```
> cargo install contracts-node --git https://github.com/paritytech/substrate-contracts-node.git --tag v0.1.0 --force --locked
```

### run node

```
> substrate-contracts-node --dev --tmp
```
![](http://cdn.hackdapp.com/2021-09-20-051306.png)

## Deployment
Use Canvas UI to deply the ERC20 contract and test. Please refer to Substrate Developer Hub for more details.

visit the canvas ui [https://paritytech.github.io/canvas-ui/#/instantiate](https://paritytech.github.io/canvas-ui/#/instantiate)

### 1. Instantiated

![](http://cdn.hackdapp.com/2021-09-20-051640.png)
![](http://cdn.hackdapp.com/2021-09-20-051658.png)

### 2. Deployed

![](http://cdn.hackdapp.com/2021-09-20-051718.png)

### 3. Transfer

![](http://cdn.hackdapp.com/2021-09-20-055758.png)
![](http://cdn.hackdapp.com/2021-09-20-055919.png)
![](http://cdn.hackdapp.com/2021-09-20-060010.png)

## FAQ
1. occur compiled or testing errors, e.g.
	![](http://cdn.hackdapp.com/2021-09-20-012420.png)
	solution: `> rustup update`

----

1. [【Ink 入门】编写并部署智能合约 ERC20 Token - 知乎](https://zhuanlan.zhihu.com/p/342576492) 使用测试网发布合约
2. CanvasNode
	[Releases · paritytech/canvas](https://github.com/paritytech/canvas/releases)
