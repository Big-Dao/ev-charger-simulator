/**
 * 正常充电流程脚本示例
 * 
 * 模拟一个完整的充电流程：
 * 1. 启动充电桩
 * 2. 开始充电
 * 3. 逐步增加功率到目标值
 * 4. 持续充电一段时间
 * 5. 停止充电
 * 
 * 可用的 JavaScript API：
 * - console.log(message)                  - 打印日志
 * - sleep(ms)                             - 异步睡眠
 * - shouldStop()                          - 检查是否应该停止脚本
 * - charger.startCharger(chargerId)       - 启动充电桩
 * - charger.stopCharger(chargerId)        - 停止充电桩
 * - charger.startCharging(chargerId)      - 开始充电
 * - charger.stopCharging(chargerId)       - 停止充电
 * - charger.setPower(chargerId, power)    - 设置功率 (kW)
 * - charger.getChargerState(chargerId)    - 获取状态 (返回 JSON 字符串)
 */

// 配置参数
const CHARGER_ID = "CP000001";
const TARGET_POWER = 30.0; // 目标功率 (kW)
const POWER_RAMP_STEP = 5.0; // 功率爬坡步长 (kW)
const POWER_RAMP_INTERVAL = 2000; // 功率爬坡间隔 (ms)
const CHARGING_DURATION = 60000; // 充电时长 (1分钟，用于演示)

console.log("=== 正常充电流程开始 ===");
console.log("充电桩 ID: " + CHARGER_ID);
console.log("目标功率: " + TARGET_POWER + " kW");

// 1. 启动充电桩
console.log("步骤 1: 启动充电桩...");
try {
    const result = charger.startCharger(CHARGER_ID);
    console.log(result);
} catch (e) {
    console.log("启动充电桩失败: " + e);
}
sleep(2000);

// 2. 开始充电
console.log("步骤 2: 开始充电...");
try {
    const result = charger.startCharging(CHARGER_ID);
    console.log(result);
} catch (e) {
    console.log("开始充电失败: " + e);
}
sleep(2000);

// 3. 功率爬坡
console.log("步骤 3: 功率爬坡...");
let currentPower = 0;
while (currentPower < TARGET_POWER && !shouldStop()) {
    currentPower += POWER_RAMP_STEP;
    if (currentPower > TARGET_POWER) {
        currentPower = TARGET_POWER;
    }
    
    try {
        const result = charger.setPower(CHARGER_ID, currentPower);
        console.log("设置功率: " + currentPower + " kW - " + result);
    } catch (e) {
        console.log("设置功率失败: " + e);
    }
    
    sleep(POWER_RAMP_INTERVAL);
}

// 4. 持续充电
console.log("步骤 4: 持续充电 " + (CHARGING_DURATION / 1000) + " 秒...");
const startTime = Date.now();
while (Date.now() - startTime < CHARGING_DURATION && !shouldStop()) {
    sleep(10000); // 每10秒检查一次
    console.log("充电中... 已充电 " + Math.round((Date.now() - startTime) / 1000) + " 秒");
}

// 5. 功率降低
console.log("步骤 5: 功率降低...");
while (currentPower > 0 && !shouldStop()) {
    currentPower -= POWER_RAMP_STEP;
    if (currentPower < 0) {
        currentPower = 0;
    }
    
    try {
        const result = charger.setPower(CHARGER_ID, currentPower);
        console.log("降低功率: " + currentPower + " kW - " + result);
    } catch (e) {
        console.log("降低功率失败: " + e);
    }
    
    sleep(1000);
}

// 6. 停止充电
console.log("步骤 6: 停止充电...");
try {
    const result = charger.stopCharging(CHARGER_ID);
    console.log(result);
} catch (e) {
    console.log("停止充电失败: " + e);
}
sleep(2000);

// 7. 停止充电桩
console.log("步骤 7: 停止充电桩...");
try {
    const result = charger.stopCharger(CHARGER_ID);
    console.log(result);
} catch (e) {
    console.log("停止充电桩失败: " + e);
}

console.log("=== 正常充电流程结束 ===");
"Script completed successfully";
main();
