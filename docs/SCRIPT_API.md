# JavaScript 脚本 API 文档

## 概述

充电桩模拟器使用 JavaScript 脚本来控制每个充电桩的行为。脚本运行在独立的沙箱环境中，通过预定义的 API 与充电桩引擎交互。

## 全局对象

### charger - 充电桩控制对象

控制充电桩的状态、功率和行为。

#### 方法

**charger.setStatus(status: string): void**

设置充电桩状态。

参数：
- `status`: 状态值，可选：
  - `'Available'` - 可用
  - `'Preparing'` - 准备中
  - `'Charging'` - 充电中
  - `'SuspendedEV'` - 暂停（车端）
  - `'SuspendedEVSE'` - 暂停（桩端）
  - `'Finishing'` - 结束中
  - `'Reserved'` - 预约
  - `'Unavailable'` - 不可用
  - `'Faulted'` - 故障

示例：
```javascript
charger.setStatus('Charging');
```

**charger.getStatus(): string**

获取当前充电桩状态。

返回值：当前状态字符串

示例：
```javascript
const status = charger.getStatus();
logger.info(`当前状态: ${status}`);
```

**charger.setPower(power: number): void**

设置充电功率。

参数：
- `power`: 功率值（kW），范围 0 到配置的最大功率

示例：
```javascript
charger.setPower(30.5); // 设置为 30.5kW
```

**charger.getCurrentPower(): number**

获取当前充电功率。

返回值：当前功率（kW）

示例：
```javascript
const power = charger.getCurrentPower();
```

**charger.startCharging(): void**

启动充电会话。会自动生成交易 ID 并通知服务器。

示例：
```javascript
charger.startCharging();
```

**charger.stopCharging(): void**

停止充电会话。会发送停止交易消息到服务器。

示例：
```javascript
charger.stopCharging();
```

**charger.updateMeterValue(): void**

上报电表读数。通常用于定时上报当前电量、功率等信息。

示例：
```javascript
// 每30秒上报一次
timer.setInterval(() => {
    charger.updateMeterValue();
}, 30000);
```

**charger.sendHeartbeat(): void**

发送心跳消息到服务器。

示例：
```javascript
// 每30秒发送心跳
timer.setInterval(() => {
    charger.sendHeartbeat();
}, 30000);
```

**charger.setError(errorCode: string): void**

设置错误代码。

参数：
- `errorCode`: 错误代码，可选：
  - `'NoError'` - 无错误
  - `'ConnectorLockFailure'` - 连接器锁定失败
  - `'EVCommunicationError'` - 与车辆通信错误
  - `'GroundFailure'` - 接地故障
  - `'HighTemperature'` - 高温
  - `'InternalError'` - 内部错误
  - `'OverCurrentFailure'` - 过流
  - `'OverVoltage'` - 过压
  - `'UnderVoltage'` - 欠压
  - `'PowerSwitchFailure'` - 电源开关故障
  - 其他...

示例：
```javascript
charger.setError('HighTemperature');
```

**charger.connect(): void**

连接到服务器。

示例：
```javascript
charger.connect();
```

**charger.disconnect(): void**

断开与服务器的连接。

示例：
```javascript
charger.disconnect();
```

**charger.getId(): string**

获取充电桩 ID。

返回值：充电桩 ID

示例：
```javascript
const id = charger.getId();
```

---

### logger - 日志对象

用于输出日志信息。

#### 方法

**logger.info(message: string): void**

输出信息级别日志。

示例：
```javascript
logger.info('充电已启动');
```

**logger.warn(message: string): void**

输出警告级别日志。

示例：
```javascript
logger.warn('功率接近上限');
```

**logger.error(message: string): void**

输出错误级别日志。

示例：
```javascript
logger.error('连接失败');
```

**logger.debug(message: string): void**

输出调试级别日志。

示例：
```javascript
logger.debug(`当前功率: ${power} kW`);
```

---

### timer - 定时器对象

提供定时任务功能。

#### 方法

**timer.setTimeout(callback: function, delay: number): number**

延时执行函数。

参数：
- `callback`: 要执行的函数
- `delay`: 延时时间（毫秒）

返回值：定时器 ID

示例：
```javascript
const timerId = timer.setTimeout(() => {
    logger.info('5秒后执行');
}, 5000);
```

**timer.setInterval(callback: function, interval: number): number**

定时重复执行函数。

参数：
- `callback`: 要执行的函数
- `interval`: 间隔时间（毫秒）

返回值：定时器 ID

示例：
```javascript
const intervalId = timer.setInterval(() => {
    charger.sendHeartbeat();
}, 30000); // 每30秒执行一次
```

**timer.clearTimeout(timerId: number): void**

清除延时定时器。

参数：
- `timerId`: 定时器 ID

示例：
```javascript
timer.clearTimeout(timerId);
```

**timer.clearInterval(intervalId: number): void**

清除间隔定时器。

参数：
- `intervalId`: 定时器 ID

示例：
```javascript
timer.clearInterval(intervalId);
```

---

## 内置对象

脚本环境支持标准的 JavaScript 内置对象：

- `Math` - 数学运算
- `Date` - 日期时间
- `JSON` - JSON 解析和序列化
- `Array`, `Object`, `String`, `Number` 等基础类型

## 脚本生命周期

1. **加载阶段**：脚本被加载和解析
2. **初始化阶段**：执行全局代码
3. **运行阶段**：响应事件和定时器
4. **清理阶段**：充电桩停止时清理资源

## 最佳实践

### 1. 初始化模式

```javascript
function initialize() {
    logger.info('初始化充电桩');
    charger.setStatus('Available');
    
    // 设置心跳
    timer.setInterval(() => {
        charger.sendHeartbeat();
    }, 30000);
}

initialize();
```

### 2. 功率爬坡

```javascript
function rampUpPower(targetPower, step, interval) {
    let currentPower = 0;
    
    const rampInterval = timer.setInterval(() => {
        currentPower += step;
        
        if (currentPower >= targetPower) {
            currentPower = targetPower;
            timer.clearInterval(rampInterval);
        }
        
        charger.setPower(currentPower);
    }, interval);
}
```

### 3. 错误处理

```javascript
function handleFault(errorCode) {
    logger.error(`故障: ${errorCode}`);
    charger.setStatus('Faulted');
    charger.setError(errorCode);
    charger.setPower(0);
    
    // 5秒后尝试恢复
    timer.setTimeout(() => {
        recoverFromFault();
    }, 5000);
}
```

### 4. 完整充电流程

```javascript
function main() {
    initialize();
    
    // 等待用户操作
    timer.setTimeout(() => {
        startCharging();
    }, 5000);
}

function startCharging() {
    charger.setStatus('Preparing');
    charger.startCharging();
    
    timer.setTimeout(() => {
        charger.setStatus('Charging');
        rampUpPower(30, 5, 2000);
    }, 2000);
}

main();
```

## 限制和约束

### 安全限制

1. **无文件系统访问**：脚本无法读写文件
2. **无网络访问**：脚本无法直接发起网络请求
3. **资源限制**：
   - 最大内存：10MB
   - 最大执行时间：每次调用 5 秒
   - 最大定时器数量：100 个

### 性能建议

1. 避免在循环中执行大量计算
2. 合理设置定时器间隔
3. 及时清理不需要的定时器
4. 使用日志的 debug 级别用于详细输出

## 示例脚本

### 基础示例

参见：
- `scripts/normal_charging.js` - 正常充电流程
- `scripts/fast_charging.js` - 快速充电
- `scripts/fault_test.js` - 异常场景测试

### 高级场景

更多示例正在开发中...

## 调试技巧

1. **使用日志**：充分使用 `logger` 对象输出调试信息
2. **状态追踪**：记录关键状态转换
3. **功率监控**：定期输出当前功率值
4. **错误捕获**：使用 try-catch 捕获异常

```javascript
try {
    charger.startCharging();
} catch (error) {
    logger.error(`启动充电失败: ${error.message}`);
}
```

## 常见问题

**Q: 如何模拟随机故障？**

A: 使用 `Math.random()` 生成随机数来触发故障：

```javascript
timer.setInterval(() => {
    if (Math.random() < 0.05) { // 5% 概率
        simulateFault();
    }
}, 60000);
```

**Q: 如何实现充电曲线？**

A: 使用数组存储功率点，然后按时间顺序设置：

```javascript
const powerCurve = [10, 20, 30, 40, 50, 45, 40, 35, 30];
let index = 0;

timer.setInterval(() => {
    if (index < powerCurve.length) {
        charger.setPower(powerCurve[index++]);
    }
}, 60000); // 每分钟改变一次
```

**Q: 脚本如何知道充电桩被手动停止？**

A: 目前脚本无法直接接收外部事件，将在后续版本中添加事件监听机制。

---

## 更新日志

- v0.1.0 (2025-10-01): 初始版本，基础 API 实现
