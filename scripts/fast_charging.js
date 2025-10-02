/**
 * 快速充电脚本示例
 * 
 * 模拟大功率快充场景：
 * - 快速爬坡到高功率
 * - 持续高功率充电
 * - 电量接近满时降低功率
 */

const MAX_POWER = 120; // 最大功率 (kW)
const HIGH_POWER_DURATION = 1800000; // 高功率充电时长 (30分钟)
const MEDIUM_POWER = 60; // 中等功率 (kW)
const MEDIUM_POWER_DURATION = 600000; // 中等功率时长 (10分钟)

logger.info('快速充电脚本已加载');

function initialize() {
    logger.info('初始化快充桩...');
    charger.setStatus('Available');
    
    // 心跳
    timer.setInterval(() => {
        charger.sendHeartbeat();
    }, 30000);
}

function startFastCharging() {
    logger.info('开始快速充电');
    charger.setStatus('Preparing');
    
    timer.setTimeout(() => {
        charger.startCharging();
        charger.setStatus('Charging');
        
        // 快速爬坡到最大功率
        rampToPower(MAX_POWER, 10, 1000);
    }, 1000);
}

function rampToPower(targetPower, step, interval) {
    let currentPower = 0;
    
    const rampInterval = timer.setInterval(() => {
        currentPower += step;
        
        if (currentPower >= targetPower) {
            currentPower = targetPower;
            timer.clearInterval(rampInterval);
            logger.info(`功率已达到: ${targetPower} kW`);
            
            if (targetPower === MAX_POWER) {
                highPowerCharging();
            } else if (targetPower === MEDIUM_POWER) {
                mediumPowerCharging();
            }
        }
        
        charger.setPower(currentPower);
    }, interval);
}

function highPowerCharging() {
    logger.info('高功率充电阶段');
    
    const meterInterval = timer.setInterval(() => {
        charger.updateMeterValue();
    }, 30000);
    
    // 切换到中等功率
    timer.setTimeout(() => {
        timer.clearInterval(meterInterval);
        logger.info('切换到中等功率');
        rampToPower(MEDIUM_POWER, 10, 500);
    }, HIGH_POWER_DURATION);
}

function mediumPowerCharging() {
    logger.info('中等功率充电阶段');
    
    const meterInterval = timer.setInterval(() => {
        charger.updateMeterValue();
    }, 30000);
    
    timer.setTimeout(() => {
        timer.clearInterval(meterInterval);
        stopFastCharging();
    }, MEDIUM_POWER_DURATION);
}

function stopFastCharging() {
    logger.info('停止快速充电');
    
    // 快速降功率
    let currentPower = charger.getCurrentPower();
    const rampDownInterval = timer.setInterval(() => {
        currentPower -= 20;
        
        if (currentPower <= 0) {
            currentPower = 0;
            timer.clearInterval(rampDownInterval);
            
            charger.stopCharging();
            charger.setPower(0);
            charger.setStatus('Finishing');
            
            timer.setTimeout(() => {
                charger.setStatus('Available');
                logger.info('快速充电完成');
            }, 2000);
        }
        
        charger.setPower(currentPower);
    }, 500);
}

function main() {
    initialize();
    
    // 随机延时后启动
    const delay = 3000 + Math.random() * 5000;
    timer.setTimeout(() => {
        startFastCharging();
    }, delay);
}

main();
