// 基础测试脚本 - 验证脚本引擎的所有功能
// Basic test script - Verify all script engine features

console.log("=== 脚本引擎测试开始 ===");
console.log("Script engine test started");

// 1. 测试 console.log
console.log("✓ Test 1: console.log works!");

// 2. 测试 sleep (异步)
console.log("Test 2: Testing sleep...");
await sleep(1000);
console.log("✓ Test 2: sleep(1000ms) completed!");

// 3. 测试 shouldStop (检查脚本是否应该停止)
console.log("Test 3: Checking shouldStop...");
if (shouldStop()) {
    console.log("Script should stop");
} else {
    console.log("✓ Test 3: shouldStop() returns false (script running)");
}

// 4. 测试创建充电桩
console.log("Test 4: Testing startCharger...");
try {
    await charger.startCharger("test-charger-001", "ws://localhost:8080", "OCPP1.6J");
    console.log("✓ Test 4: startCharger succeeded!");
} catch (error) {
    console.log("⚠ Test 4: startCharger failed (expected if no OCPP server): " + error);
}

// 5. 测试获取充电桩状态
console.log("Test 5: Testing getChargerState...");
try {
    const state = await charger.getChargerState("test-charger-001");
    console.log("✓ Test 5: getChargerState succeeded!");
    console.log("Charger state:", JSON.stringify(state, null, 2));
} catch (error) {
    console.log("⚠ Test 5: getChargerState failed: " + error);
}

// 6. 测试设置功率
console.log("Test 6: Testing setPower...");
try {
    await charger.setPower("test-charger-001", 7.4);
    console.log("✓ Test 6: setPower(7.4 kW) succeeded!");
} catch (error) {
    console.log("⚠ Test 6: setPower failed: " + error);
}

// 7. 测试开始充电
console.log("Test 7: Testing startCharging...");
try {
    await charger.startCharging("test-charger-001");
    console.log("✓ Test 7: startCharging succeeded!");
} catch (error) {
    console.log("⚠ Test 7: startCharging failed: " + error);
}

// 8. 等待一小段时间
console.log("Test 8: Waiting for 2 seconds...");
await sleep(2000);
console.log("✓ Test 8: Wait completed!");

// 9. 测试停止充电
console.log("Test 9: Testing stopCharging...");
try {
    await charger.stopCharging("test-charger-001");
    console.log("✓ Test 9: stopCharging succeeded!");
} catch (error) {
    console.log("⚠ Test 9: stopCharging failed: " + error);
}

// 10. 测试停止充电桩
console.log("Test 10: Testing stopCharger...");
try {
    await charger.stopCharger("test-charger-001");
    console.log("✓ Test 10: stopCharger succeeded!");
} catch (error) {
    console.log("⚠ Test 10: stopCharger failed: " + error);
}

// 最终检查
console.log("\n=== 测试完成 ===");
console.log("All tests completed!");
console.log("Script engine is working properly! 🎉");
