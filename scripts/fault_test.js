/**
 * 异常场景测试脚本
 * 
 * 模拟各种异常情况：
 * - 网络断开重连
 * - 充电异常中断
 * - 过温保护
 * - 故障恢复
 */

const NORMAL_POWER = 30;
const FAULT_SIMULATION_DELAY = 60000; // 1分钟后模拟故障

logger.info('异常场景测试脚本已加载');

let faultSimulated = false;

function initialize() {
    logger.info('初始化充电桩（异常测试模式）...');
    charger.setStatus('Available');
    
    timer.setInterval(() => {
        charger.sendHeartbeat();
    }, 30000);
}

function startCharging() {
    logger.info('开始充电');
    charger.setStatus('Preparing');
    
    timer.setTimeout(() => {
        charger.startCharging();
        charger.setStatus('Charging');
        
        // 爬坡到正常功率
        rampUpPower();
        
        // 定时模拟故障
        timer.setTimeout(() => {
            simulateFault();
        }, FAULT_SIMULATION_DELAY);
    }, 2000);
}

function rampUpPower() {
    let power = 0;
    const interval = timer.setInterval(() => {
        power += 5;
        if (power >= NORMAL_POWER) {
            power = NORMAL_POWER;
            timer.clearInterval(interval);
        }
        charger.setPower(power);
    }, 1000);
}

function simulateFault() {
    if (faultSimulated) return;
    faultSimulated = true;
    
    const faultType = Math.floor(Math.random() * 3);
    
    switch (faultType) {
        case 0:
            simulateOverTemperature();
            break;
        case 1:
            simulateConnectionLoss();
            break;
        case 2:
            simulatePowerFailure();
            break;
    }
}

function simulateOverTemperature() {
    logger.warn('检测到过温故障！');
    charger.setStatus('Faulted');
    charger.setError('HighTemperature');
    
    // 立即降低功率
    const currentPower = charger.getCurrentPower();
    let power = currentPower;
    const interval = timer.setInterval(() => {
        power -= 10;
        if (power <= 0) {
            power = 0;
            timer.clearInterval(interval);
            
            // 5秒后恢复
            timer.setTimeout(() => {
                recoverFromFault();
            }, 5000);
        }
        charger.setPower(power);
    }, 500);
}

function simulateConnectionLoss() {
    logger.error('网络连接丢失！');
    charger.disconnect();
    
    // 10秒后重连
    timer.setTimeout(() => {
        logger.info('尝试重新连接...');
        charger.connect();
        
        timer.setTimeout(() => {
            logger.info('重连成功，恢复充电');
            recoverFromFault();
        }, 2000);
    }, 10000);
}

function simulatePowerFailure() {
    logger.error('检测到供电故障！');
    charger.setStatus('Faulted');
    charger.setError('PowerSwitchFailure');
    charger.setPower(0);
    
    // 8秒后恢复
    timer.setTimeout(() => {
        logger.info('供电已恢复');
        recoverFromFault();
    }, 8000);
}

function recoverFromFault() {
    logger.info('故障已恢复，准备恢复充电');
    charger.setStatus('Available');
    charger.setError('NoError');
    
    // 重新开始充电
    timer.setTimeout(() => {
        startCharging();
    }, 2000);
}

function main() {
    initialize();
    
    timer.setTimeout(() => {
        startCharging();
    }, 5000);
}

main();
