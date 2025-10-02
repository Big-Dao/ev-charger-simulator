<template>
  <a-config-provider :theme="themeConfig">
    <div class="app-container" :class="{ 'dark-mode': isDark, 'light-mode': !isDark }">
      <a-layout class="layout">
        <!-- 头部 -->
        <a-layout-header class="header">
          <div class="header-content">
            <div class="logo">
              <ThunderboltOutlined class="logo-icon" />
              <span class="logo-text">充电桩模拟器</span>
            </div>
            <div class="header-actions">
              <a-space>
                <a-button type="text" size="large" style="color: #fff;">
                  <CheckCircleOutlined style="color: #52c41a;" />
                  在线: {{ onlineCount }}
                </a-button>
                <a-button type="text" size="large" style="color: #fff;">
                  <ThunderboltOutlined style="color: #1890ff;" />
                  充电中: {{ chargingCount }}
                </a-button>
                <a-switch v-model:checked="isDark" checked-children="暗色" un-checked-children="亮色" />
              </a-space>
            </div>
          </div>
        </a-layout-header>

        <!-- 主体内容 -->
        <a-layout-content class="content">
          <div class="content-inner">
            <!-- 统计卡片区域 -->
            <a-card class="welcome-card" :bordered="false">
              <a-row :gutter="[12, 12]">
                <a-col :xs="24" :sm="12" :md="6">
                  <a-card class="stat-card">
                    <a-statistic
                      title="总桩数"
                      :value="totalCount"
                      :value-style="{ color: '#1890ff' }"
                    >
                      <template #prefix>
                        <DatabaseOutlined />
                      </template>
                    </a-statistic>
                  </a-card>
                </a-col>
                <a-col :xs="24" :sm="12" :md="6">
                  <a-card class="stat-card">
                    <a-statistic
                      title="在线数量"
                      :value="onlineCount"
                      :value-style="{ color: '#52c41a' }"
                    >
                      <template #prefix>
                        <CheckCircleOutlined />
                      </template>
                    </a-statistic>
                  </a-card>
                </a-col>
                <a-col :xs="24" :sm="12" :md="6">
                  <a-card class="stat-card">
                    <a-statistic
                      title="充电中"
                      :value="chargingCount"
                      :value-style="{ color: '#faad14' }"
                    >
                      <template #prefix>
                        <PoweroffOutlined />
                      </template>
                    </a-statistic>
                  </a-card>
                </a-col>
                <a-col :xs="24" :sm="12" :md="6">
                  <a-card class="stat-card">
                    <a-statistic
                      title="总充电功率"
                      :value="status.totalPower"
                      suffix="kW"
                      :value-style="{ color: '#722ed1' }"
                    >
                      <template #prefix>
                        <DashboardOutlined />
                      </template>
                    </a-statistic>
                  </a-card>
                </a-col>
              </a-row>
            </a-card>

            <!-- 快速操作 -->
                    <a-card title="快速操作" :bordered="false" class="action-card">
                      <a-space size="small" wrap>
                        <a-button
                          type="primary"
                          :loading="globalLoading.startAll"
                          @click="startSimulation"
                        >
                          <template #icon><PlayCircleOutlined /></template>
                          启动所有模拟
                        </a-button>
                        <a-button
                          danger
                          :loading="globalLoading.stopAll"
                          @click="stopSimulation"
                        >
                          <template #icon><PauseCircleOutlined /></template>
                          停止所有模拟
                        </a-button>
                        <a-button
                          :loading="globalLoading.reset"
                          @click="resetSimulation"
                        >
                          <template #icon><ReloadOutlined /></template>
                          重置
                        </a-button>
                        <a-divider type="vertical" />
                        <a-button
                          type="primary"
                          ghost
                          @click="openAddChargerModal"
                        >
                          <template #icon><PlusOutlined /></template>
                          添加充电桩
                        </a-button>
                        <a-divider type="vertical" />
                        <a-button
                          type="dashed"
                          :loading="scriptTestLoading"
                          @click="testScriptEngine"
                        >
                          <template #icon><ThunderboltOutlined /></template>
                          测试脚本引擎
                        </a-button>
                      </a-space>
                    </a-card>

            <!-- 充电桩列表 -->
            <a-card title="充电桩列表" :bordered="false" class="charger-card">
              <template #extra>
                <span style="font-size: 12px; color: #8c8c8c;">
                  共 {{ chargers.length }} 台
                </span>
              </template>
              <a-table
                class="charger-table"
                size="small"
                :columns="chargerTableColumns"
                :data-source="chargers"
                :row-key="(record: ChargerStateView) => record.chargerId"
                :loading="listLoading"
                :pagination="tablePagination"
                @change="handleTableChange"
                :scroll="{ x: 1200 }"
                :expandable="{ defaultExpandAllRows: false }"
              >
                <template #bodyCell="{ column, record, text }">
                  <template v-if="column.key === 'connection'">
                    <a-badge 
                      :status="record.connected ? 'success' : 'default'" 
                      :text="record.connected ? '已连接' : '未连接'" 
                    />
                  </template>

                  <template v-else-if="column.key === 'status'">
                    <a-badge :status="getStatusBadge(record.status)" :text="record.status" />
                  </template>

                  <template v-else-if="column.key === 'scriptStatus'">
                    <a-space size="small">
                      <a-badge 
                        :status="record.scriptRunning ? 'processing' : 'default'" 
                        :text="record.scriptRunning ? '运行中' : '已停止'" 
                      />
                      <a-tooltip v-if="record.scriptName" :title="record.scriptName">
                        <FileTextOutlined style="color: #1890ff; cursor: help;" />
                      </a-tooltip>
                    </a-space>
                  </template>

                  <template v-else-if="column.key === 'currentPower'">
                    <span style="white-space: nowrap;">{{ record.currentPower.toFixed(1) }} kW</span>
                  </template>

                  <template v-else-if="column.key === 'totalEnergy'">
                    <span style="white-space: nowrap;">{{ record.totalEnergy.toFixed(2) }} kWh</span>
                  </template>

                  <template v-else-if="column.key === 'lastUpdated'">
                    <span style="white-space: nowrap; font-size: 12px;">{{ formatDateTime(record.lastUpdated) }}</span>
                  </template>

                  <template v-else-if="column.key === 'operations'">
                    <div class="table-operations">
                      <a-tooltip title="启动">
                        <a-button
                          size="small"
                          type="primary"
                          shape="circle"
                          :loading="isActionLoading(record.chargerId, 'start')"
                          @click="startSingleCharger(record.chargerId)"
                        >
                          <template #icon><PlayCircleOutlined /></template>
                        </a-button>
                      </a-tooltip>
                      <a-tooltip title="停止">
                        <a-button
                          size="small"
                          danger
                          shape="circle"
                          :loading="isActionLoading(record.chargerId, 'stop')"
                          @click="stopSingleCharger(record.chargerId)"
                        >
                          <template #icon><PauseCircleOutlined /></template>
                        </a-button>
                      </a-tooltip>
                      <a-tooltip title="重置">
                        <a-button
                          size="small"
                          shape="circle"
                          :loading="isActionLoading(record.chargerId, 'reset')"
                          @click="resetCharger(record.chargerId)"
                        >
                          <template #icon><ReloadOutlined /></template>
                        </a-button>
                      </a-tooltip>
                      <a-tooltip title="开始充电">
                        <a-button
                          size="small"
                          type="primary"
                          ghost
                          shape="circle"
                          :loading="isActionLoading(record.chargerId, 'startCharging')"
                          @click="startChargingSession(record.chargerId)"
                        >
                          <template #icon><ThunderboltOutlined /></template>
                        </a-button>
                      </a-tooltip>
                      <a-tooltip title="停止充电">
                        <a-button
                          size="small"
                          danger
                          ghost
                          shape="circle"
                          :loading="isActionLoading(record.chargerId, 'stopCharging')"
                          @click="stopChargingSession(record.chargerId)"
                        >
                          <template #icon><PoweroffOutlined /></template>
                        </a-button>
                      </a-tooltip>
                      <a-divider type="vertical" style="height: 24px; margin: 0 4px;" />
                      <a-tooltip :title="record.scriptRunning ? '停止脚本' : '启动脚本'">
                        <a-button
                          size="small"
                          :type="record.scriptRunning ? 'default' : 'primary'"
                          :ghost="!record.scriptRunning"
                          shape="circle"
                          :loading="isActionLoading(record.chargerId, record.scriptRunning ? 'stopScript' : 'startScript')"
                          :disabled="!record.scriptName"
                          @click="record.scriptRunning ? stopScript(record.chargerId) : startScript(record.chargerId)"
                        >
                          <template #icon>
                            <CodeOutlined v-if="!record.scriptRunning" />
                            <PauseCircleOutlined v-else />
                          </template>
                        </a-button>
                      </a-tooltip>
                      <a-tooltip title="配置脚本">
                        <a-button
                          size="small"
                          type="dashed"
                          shape="circle"
                          @click="openScriptConfig(record)"
                        >
                          <template #icon><SettingOutlined /></template>
                        </a-button>
                      </a-tooltip>
                      <a-tooltip title="充电桩参数">
                        <a-button
                          size="small"
                          type="default"
                          shape="circle"
                          @click="openConfigModal(record)"
                        >
                          <template #icon><ControlOutlined /></template>
                        </a-button>
                      </a-tooltip>
                      <a-divider type="vertical" style="height: 24px; margin: 0 4px;" />
                      <a-popconfirm
                        title="确定要删除这个充电桩吗？"
                        ok-text="确定"
                        cancel-text="取消"
                        @confirm="removeCharger(record.chargerId)"
                      >
                        <a-tooltip title="删除充电桩">
                          <a-button
                            size="small"
                            danger
                            shape="circle"
                            :loading="isActionLoading(record.chargerId, 'remove')"
                          >
                            <template #icon><DeleteOutlined /></template>
                          </a-button>
                        </a-tooltip>
                      </a-popconfirm>
                    </div>
                  </template>

                  <template v-else>
                    {{ text }}
                  </template>
                </template>

                <template #expandedRowRender="{ record }">
                  <div class="expanded-row">
                    <a-descriptions :column="2" size="small" bordered>
                      <a-descriptions-item label="会话 ID">
                        <a-tag v-if="record.transactionId" color="blue">{{ record.transactionId }}</a-tag>
                        <span v-else style="color: #8c8c8c;">—</span>
                      </a-descriptions-item>
                      <a-descriptions-item label="错误代码">
                        <a-tag v-if="record.errorCode" color="red">{{ record.errorCode }}</a-tag>
                        <span v-else style="color: #52c41a;">正常</span>
                      </a-descriptions-item>
                      <a-descriptions-item label="协议类型">
                        <a-tag color="cyan">{{ chargerConfigs[record.chargerId]?.protocol_type || 'OCPP' }}</a-tag>
                      </a-descriptions-item>
                      <a-descriptions-item label="最大功率">
                        <span style="color: #722ed1; font-weight: 600;">{{ chargerConfigs[record.chargerId]?.max_power || 60 }} kW</span>
                      </a-descriptions-item>
                      <a-descriptions-item label="脚本信息" :span="2">
                        <div v-if="record.scriptName" style="display: flex; flex-direction: column; gap: 8px;">
                          <a-space>
                            <a-tag color="blue">{{ record.scriptName }}</a-tag>
                            <a-tag v-if="record.scriptRunning" color="processing">运行中</a-tag>
                            <a-tag v-else color="default">已停止</a-tag>
                          </a-space>
                        </div>
                        <a-space v-else>
                          <span style="color: #8c8c8c;">未配置脚本</span>
                          <a-button size="small" type="link" @click="openScriptConfig(record)">
                            点击配置
                          </a-button>
                        </a-space>
                      </a-descriptions-item>
                      <a-descriptions-item label="最近执行结果" :span="2">
                        <a-space v-if="record.scriptLastSuccess !== null">
                          <a-tag :color="record.scriptLastSuccess ? 'success' : 'error'">
                            {{ record.scriptLastSuccess ? '成功' : '失败' }}
                          </a-tag>
                          <span v-if="record.scriptLastMessage" style="font-size: 12px; color: #595959;">
                            {{ record.scriptLastMessage }}
                          </span>
                        </a-space>
                        <span v-else style="color: #8c8c8c;">暂无执行记录</span>
                      </a-descriptions-item>
                    </a-descriptions>
                  </div>
                </template>
              </a-table>
            </a-card>

            <!-- 使用说明 -->
            <a-card title="快速上手" :bordered="false" class="guide-card">
              <a-steps direction="vertical" size="small">
                <a-step title="1. 添加充电桩" status="process">
                  <template #description>
                    <div style="display: flex; flex-direction: column; gap: 4px;">
                      <span>点击"添加充电桩"按钮创建虚拟充电桩</span>
                      <span style="font-size: 11px; color: #8c8c8c;">支持 OCPP 1.6J 和云快充协议</span>
                    </div>
                  </template>
                </a-step>
                <a-step title="2. 配置充电桩参数">
                  <template #description>
                    <div style="display: flex; flex-direction: column; gap: 4px;">
                      <span>设置充电桩名称、协议类型、服务器地址和最大功率</span>
                      <span style="font-size: 11px; color: #8c8c8c;">点击操作栏的 <ControlOutlined style="font-size: 11px;" /> 参数按钮进行配置</span>
                    </div>
                  </template>
                </a-step>
                <a-step title="3. 配置脚本（可选）">
                  <template #description>
                    <div style="display: flex; flex-direction: column; gap: 4px;">
                      <span>选择预设脚本或编写自定义 JavaScript 脚本</span>
                      <span style="font-size: 11px; color: #8c8c8c;">点击 <SettingOutlined style="font-size: 11px;" /> 脚本按钮，可选：基础测试、正常充电、快充、故障测试</span>
                    </div>
                  </template>
                </a-step>
                <a-step title="4. 启动模拟">
                  <template #description>
                    <div style="display: flex; flex-direction: column; gap: 4px;">
                      <span>点击 <PlayCircleOutlined style="font-size: 11px;" /> 启动按钮连接到充电站服务器</span>
                      <span style="font-size: 11px; color: #8c8c8c;">启动后可以手动控制充电，或自动运行脚本</span>
                    </div>
                  </template>
                </a-step>
                <a-step title="5. 切换主题">
                  <template #description>
                    <div style="display: flex; flex-direction: column; gap: 4px;">
                      <span>右上角可切换亮色/暗色主题，配置自动保存</span>
                      <span style="font-size: 11px; color: #8c8c8c;">所有操作都会自动持久化，重启后恢复</span>
                    </div>
                  </template>
                </a-step>
              </a-steps>
              
              <a-divider style="margin: 16px 0;" />
              
              <div style="display: flex; flex-direction: column; gap: 8px; font-size: 12px;">
                <div style="font-weight: 600; color: #1890ff;">💡 提示</div>
                <ul style="margin: 0; padding-left: 20px; color: #595959;">
                  <li>充电桩列表支持分页和排序</li>
                  <li>所有修改（增删改）自动保存到 config/chargers.json</li>
                  <li>点击充电桩行可展开查看详细信息</li>
                  <li>脚本可设置为自动启动，随充电桩连接自动运行</li>
                  <li>窗口位置和大小会自动记忆</li>
                </ul>
              </div>
            </a-card>
          </div>
        </a-layout-content>

        <!-- 底部 -->
        <a-layout-footer class="footer">
          <div class="footer-content">
            <span>EV Charger Simulator © 2025</span>
            <span>基于 Tauri + Vue 3 + Ant Design Vue</span>
          </div>
        </a-layout-footer>
      </a-layout>
    </div>

    <a-modal
      v-model:open="scriptModalVisible"
      title="配置充电桩脚本"
      :confirm-loading="scriptForm.submitting"
      width="800px"
      @ok="submitScriptForm"
      @cancel="closeScriptModal"
    >
      <a-form layout="vertical">
        <a-form-item label="充电桩 ID">
          <a-input :value="scriptForm.chargerId" disabled />
        </a-form-item>
        
        <a-form-item label="选择预设脚本">
          <a-select 
            v-model:value="scriptForm.selectedPreset" 
            placeholder="选择一个预设脚本模板"
            @change="loadPresetScript"
            allow-clear
          >
            <a-select-option value="">
              <em>自定义脚本</em>
            </a-select-option>
            <a-select-option 
              v-for="preset in presetScripts" 
              :key="preset.key" 
              :value="preset.key"
            >
              <FileTextOutlined /> {{ preset.name }} - {{ preset.description }}
            </a-select-option>
          </a-select>
        </a-form-item>

        <a-form-item label="脚本名称">
          <a-input v-model:value="scriptForm.name" placeholder="请输入脚本名称" />
        </a-form-item>
        
        <a-form-item label="自动随充电桩启动脚本">
          <a-switch
            v-model:checked="scriptForm.autoStart"
            checked-children="启用"
            un-checked-children="关闭"
          />
          <span style="margin-left: 12px; color: #8c8c8c; font-size: 12px;">
            启用后，充电桩连接成功时将自动运行脚本
          </span>
        </a-form-item>
        
        <a-form-item label="脚本代码">
          <a-textarea
            v-model:value="scriptForm.code"
            :rows="12"
            placeholder="在此粘贴或编辑脚本代码..."
            style="font-family: 'Consolas', 'Monaco', 'Courier New', monospace; font-size: 13px;"
          />
        </a-form-item>
        
        <a-alert
          message="提示"
          description="您可以选择预设脚本模板，或者编写自定义脚本。脚本将在充电桩启动时执行，用于模拟各种充电场景。"
          type="info"
          show-icon
          :closable="false"
          style="margin-top: 8px;"
        />
      </a-form>
    </a-modal>

    <!-- 添加充电桩对话框 -->
    <a-modal
      v-model:open="addChargerModalVisible"
      title="添加新充电桩"
      :confirm-loading="addChargerForm.submitting"
      width="700px"
      @ok="submitAddChargerForm"
      @cancel="closeAddChargerModal"
    >
      <a-form layout="vertical" :label-col="{ span: 24 }">
        <a-row :gutter="16">
          <a-col :span="12">
            <a-form-item label="充电桩 ID" required>
              <a-input
                v-model:value="addChargerForm.id"
                placeholder="例：CP000010"
              >
                <template #prefix>🆔</template>
              </a-input>
            </a-form-item>
          </a-col>
          <a-col :span="12">
            <a-form-item label="充电桩名称" required>
              <a-input
                v-model:value="addChargerForm.name"
                placeholder="例：10号充电桩"
              />
            </a-form-item>
          </a-col>
        </a-row>

        <a-row :gutter="16">
          <a-col :span="12">
            <a-form-item label="协议类型" required>
              <a-select v-model:value="addChargerForm.protocol_type" placeholder="选择协议类型">
                <a-select-option value="OCPP">
                  <CloudOutlined /> OCPP 1.6J
                </a-select-option>
                <a-select-option value="YunKuaiChong">
                  <ApiOutlined /> 云快充协议
                </a-select-option>
              </a-select>
            </a-form-item>
          </a-col>
          <a-col :span="12">
            <a-form-item label="最大功率 (kW)" required>
              <a-input-number
                v-model:value="addChargerForm.max_power"
                :min="1"
                :max="500"
                :step="10"
                style="width: 100%;"
                placeholder="最大功率"
              />
            </a-form-item>
          </a-col>
        </a-row>

        <a-form-item label="服务器地址" required>
          <a-input
            v-model:value="addChargerForm.server_url"
            placeholder="ws://localhost:8080/ocpp"
          >
            <template #prefix>
              <LinkOutlined />
            </template>
          </a-input>
        </a-form-item>

        <a-form-item label="启用状态">
          <a-switch
            v-model:checked="addChargerForm.enabled"
            checked-children="启用"
            un-checked-children="禁用"
          />
          <span style="margin-left: 12px; color: #8c8c8c; font-size: 12px;">
            启用后将自动加入充电桩列表
          </span>
        </a-form-item>

        <a-alert
          message="提示"
          description="添加充电桩后，如果启用了自动启动，充电桩将立即尝试连接到服务器。请确保服务器地址正确。"
          type="info"
          show-icon
          :closable="false"
        />
      </a-form>
    </a-modal>

    <!-- 充电桩参数配置对话框 -->
    <a-modal
      v-model:open="configModalVisible"
      title="充电桩参数配置"
      :confirm-loading="configForm.submitting"
      width="700px"
      @ok="submitConfigForm"
      @cancel="closeConfigModal"
    >
      <a-form layout="vertical" :label-col="{ span: 24 }">
        <a-row :gutter="16">
          <a-col :span="12">
            <a-form-item label="充电桩 ID">
              <a-input :value="configForm.id" disabled />
            </a-form-item>
          </a-col>
          <a-col :span="12">
            <a-form-item label="充电桩名称">
              <a-input v-model:value="configForm.name" placeholder="请输入充电桩名称" />
            </a-form-item>
          </a-col>
        </a-row>

        <a-row :gutter="16">
          <a-col :span="12">
            <a-form-item label="协议类型">
              <a-select v-model:value="configForm.protocol_type" placeholder="选择协议类型">
                <a-select-option value="OCPP">
                  <CloudOutlined /> OCPP 1.6J
                </a-select-option>
                <a-select-option value="YunKuaiChong">
                  <ApiOutlined /> 云快充协议
                </a-select-option>
              </a-select>
            </a-form-item>
          </a-col>
          <a-col :span="12">
            <a-form-item label="最大功率 (kW)">
              <a-input-number
                v-model:value="configForm.max_power"
                :min="1"
                :max="500"
                :step="1"
                style="width: 100%;"
                placeholder="最大功率"
              />
            </a-form-item>
          </a-col>
        </a-row>

        <a-form-item label="服务器地址">
          <a-input
            v-model:value="configForm.server_url"
            placeholder="ws://localhost:8080/ocpp 或 https://api.yunkuaichong.com"
          >
            <template #prefix>
              <LinkOutlined />
            </template>
          </a-input>
        </a-form-item>

        <a-form-item label="启用状态">
          <a-switch
            v-model:checked="configForm.enabled"
            checked-children="启用"
            un-checked-children="禁用"
          />
          <span style="margin-left: 12px; color: #8c8c8c; font-size: 12px;">
            禁用后充电桩将不会自动启动
          </span>
        </a-form-item>

        <a-alert
          message="提示"
          description="修改协议类型或服务器地址后，充电桩将自动重启以应用新配置。如果充电桩正在充电，请先停止充电。"
          type="warning"
          show-icon
          :closable="false"
        />
      </a-form>
    </a-modal>
  </a-config-provider>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onBeforeUnmount } from 'vue';
import { theme } from 'ant-design-vue';
import {
  ThunderboltOutlined,
  CheckCircleOutlined,
  PoweroffOutlined,
  DashboardOutlined,
  DatabaseOutlined,
  PlayCircleOutlined,
  PauseCircleOutlined,
  ReloadOutlined,
  CodeOutlined,
  FileTextOutlined,
  SettingOutlined,
  ControlOutlined,
  CloudOutlined,
  ApiOutlined,
  LinkOutlined,
  PlusOutlined,
  DeleteOutlined,
} from '@ant-design/icons-vue';
import { message } from 'ant-design-vue';
import { invoke } from '@tauri-apps/api/tauri';
import testScriptSource from '../scripts/basic_test.js?raw';

type RawChargerState = {
  charger_id: string;
  status: string;
  error_code: string;
  connected: boolean;
  current_power: number;
  total_energy: number;
  transaction_id: number | null;
  last_updated: string;
  script_name?: string | null;
  script_running: boolean;
  script_last_success?: boolean | null;
  script_last_message?: string | null;
};

type ChargerStateView = {
  chargerId: string;
  status: string;
  errorCode: string;
  connected: boolean;
  currentPower: number;
  totalEnergy: number;
  transactionId: number | null;
  lastUpdated: string;
  scriptName: string | null;
  scriptRunning: boolean;
  scriptLastSuccess: boolean | null;
  scriptLastMessage: string | null;
};

type ChargerConfig = {
  id: string;
  name: string;
  protocol_type: string;
  server_url: string;
  max_power: number;
  script_path: string | null;
  enabled: boolean;
};

type StatisticsResponse = {
  total_chargers: number;
  online_count: number;
  charging_count: number;
  total_power: number;
};

// 主题模式（从 localStorage 读取保存的配置）
const isDark = ref(localStorage.getItem('theme-mode') === 'dark');

const themeConfig = computed(() => ({
  algorithm: isDark.value ? theme.darkAlgorithm : theme.defaultAlgorithm,
  token: {
    colorPrimary: '#1890ff',
    borderRadius: 8,
  },
}));

watch(isDark, (value) => {
  // 保存主题配置到 localStorage
  localStorage.setItem('theme-mode', value ? 'dark' : 'light');
  message.success(`已切换到${value ? '暗色' : '亮色'}模式`);
});

const status = ref({
  systemStatus: '就绪',
  description: '系统已准备就绪，等待启动模拟',
  totalChargers: 0,
  totalPower: 0,
});

const onlineCount = ref(0);
const chargingCount = ref(0);

const chargers = ref<ChargerStateView[]>([]);
const chargerConfigs = ref<Record<string, ChargerConfig>>({});
const listLoading = ref(false);
const autoRefresh = ref(true);
let refreshTimer: ReturnType<typeof setInterval> | undefined;

type ChargerTableColumn = {
  title: string;
  dataIndex?: string;
  key: string;
  align?: 'left' | 'right' | 'center';
  width?: number;
  fixed?: 'left' | 'right';
};

type ChargerTablePagination = {
  current: number;
  pageSize: number | string;
  total: number;
  showSizeChanger: boolean;
  pageSizeOptions: string[];
  showLessItems: boolean;
  showTotal: (total: number, range: [number, number]) => string;
};

const tablePagination = ref<ChargerTablePagination>({
  current: 1,
  pageSize: 20,
  total: 0,
  showSizeChanger: true,
  pageSizeOptions: ['10', '20', '50', '100'],
  showLessItems: true,
  showTotal: (total, range) => `${range[0]}-${range[1]} / ${total}`,
});

const chargerTableColumns = computed<ChargerTableColumn[]>(() => [
  {
    title: '充电桩ID',
    dataIndex: 'chargerId',
    key: 'charger',
    width: 120,
    fixed: 'left',
  },
  {
    title: '连接状态',
    dataIndex: 'connected',
    key: 'connection',
    align: 'center',
    width: 100,
  },
  {
    title: '运行状态',
    dataIndex: 'status',
    key: 'status',
    align: 'center',
    width: 110,
  },
  {
    title: '脚本状态',
    dataIndex: 'scriptRunning',
    key: 'scriptStatus',
    align: 'center',
    width: 100,
  },
  {
    title: '当前功率',
    dataIndex: 'currentPower',
    key: 'currentPower',
    align: 'right',
    width: 100,
  },
  {
    title: '累计电量',
    dataIndex: 'totalEnergy',
    key: 'totalEnergy',
    align: 'right',
    width: 100,
  },
  {
    title: '最后更新时间',
    dataIndex: 'lastUpdated',
    key: 'lastUpdated',
    width: 160,
  },
  {
    title: '操作',
    key: 'operations',
    width: 380,
    fixed: 'right',
  },
]);

const normalizePageSize = (value: number | string | undefined) => {
  if (typeof value === 'string') {
    const parsed = Number(value);
    return Number.isFinite(parsed) && parsed > 0 ? parsed : 20;
  }
  return value && value > 0 ? value : 20;
};

const syncPaginationTotal = (total: number) => {
  const pageSize = normalizePageSize(tablePagination.value.pageSize);
  const totalSafe = Math.max(total, 0);
  const maxPage = Math.max(1, Math.ceil((totalSafe || 1) / pageSize));
  const current = Math.min(tablePagination.value.current || 1, maxPage);
  tablePagination.value = {
    ...tablePagination.value,
    current,
    pageSize,
    total: totalSafe,
  };
};

const handleTableChange = (pager: { current?: number; pageSize?: number }) => {
  const nextPageSize = normalizePageSize(pager.pageSize ?? tablePagination.value.pageSize);
  const nextCurrent = pager.current ?? tablePagination.value.current;
  tablePagination.value = {
    ...tablePagination.value,
    current: nextCurrent ?? 1,
    pageSize: nextPageSize,
  };
  syncPaginationTotal(chargers.value.length);
};

const actionLoading = ref<Record<string, Record<string, boolean>>>({});
const powerDrafts = ref<Record<string, number>>({});

const globalLoading = ref({
  startAll: false,
  stopAll: false,
  reset: false,
});

const scriptTestLoading = ref(false);

const scriptModalVisible = ref(false);
const scriptForm = ref({
  chargerId: '',
  name: '',
  autoStart: true,
  code: '',
  submitting: false,
  selectedPreset: '',
});

const configModalVisible = ref(false);
const configForm = ref({
  id: '',
  name: '',
  protocol_type: 'OCPP',
  server_url: '',
  max_power: 60,
  script_path: null as string | null,
  enabled: true,
  submitting: false,
});

const addChargerModalVisible = ref(false);
const addChargerForm = ref({
  id: '',
  name: '',
  protocol_type: 'OCPP',
  server_url: 'ws://localhost:8080/ocpp',
  max_power: 60,
  script_path: null as string | null,
  enabled: true,
  submitting: false,
});

const getStatusBadge = (state: string): 'success' | 'processing' | 'default' | 'error' | 'warning' => {
  const badgeMap: Record<string, 'success' | 'processing' | 'default' | 'error' | 'warning'> = {
    Available: 'success',
    Preparing: 'processing',
    Charging: 'processing',
    SuspendedEV: 'warning',
    SuspendedEVSE: 'warning',
    Finishing: 'processing',
    Reserved: 'default',
    Unavailable: 'default',
    Faulted: 'error',
  };
  return badgeMap[state] || 'default';
};

const totalCount = computed(() => status.value.totalChargers);

const errorMessage = (error: unknown) =>
  error instanceof Error ? error.message : String(error);

// 格式化时间显示
const formatDateTime = (isoString: string): string => {
  try {
    const date = new Date(isoString);
    if (isNaN(date.getTime())) {
      return '无效时间';
    }
    
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0');
    const day = String(date.getDate()).padStart(2, '0');
    const hours = String(date.getHours()).padStart(2, '0');
    const minutes = String(date.getMinutes()).padStart(2, '0');
    const seconds = String(date.getSeconds()).padStart(2, '0');
    
    return `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`;
  } catch (e) {
    return '无效时间';
  }
};

const mapChargerState = (raw: RawChargerState): ChargerStateView => ({
  chargerId: raw.charger_id,
  status: raw.status,
  errorCode: raw.error_code,
  connected: raw.connected,
  currentPower: raw.current_power,
  totalEnergy: raw.total_energy,
  transactionId: raw.transaction_id,
  lastUpdated: raw.last_updated,
  scriptName: raw.script_name ?? null,
  scriptRunning: raw.script_running,
  scriptLastSuccess: raw.script_last_success ?? null,
  scriptLastMessage: raw.script_last_message ?? null,
});

const ensurePowerDrafts = (list: ChargerStateView[]) => {
  const next: Record<string, number> = {};
  list.forEach((item) => {
    const existing = powerDrafts.value[item.chargerId];
    next[item.chargerId] = existing ?? Number(item.currentPower.toFixed(1));
  });
  powerDrafts.value = next;
};

const updateStatistics = (stats: StatisticsResponse) => {
  const total = stats.total_chargers ?? 0;
  const online = stats.online_count ?? 0;
  const charging = stats.charging_count ?? 0;
  const power = Number((stats.total_power ?? 0).toFixed(2));

  status.value.totalChargers = total;
  status.value.totalPower = power;
  onlineCount.value = online;
  chargingCount.value = charging;

  if (charging > 0) {
    status.value.systemStatus = '运行中';
    status.value.description = `共有 ${charging} 台充电桩正在充电`;
  } else if (online > 0) {
    status.value.systemStatus = '待机';
    status.value.description = `共有 ${online} 台充电桩在线等待调度`;
  } else {
    status.value.systemStatus = '就绪';
    status.value.description = '系统已准备就绪，等待启动模拟';
  }
};

const refreshDashboard = async (showToast = false) => {
  listLoading.value = true;
  try {
    const [listRaw, stats] = await Promise.all([
      invoke<RawChargerState[]>('get_charger_list'),
      invoke<StatisticsResponse>('get_statistics'),
    ]);
    const mapped = listRaw.map(mapChargerState);
    chargers.value = mapped;
    console.log('Chargers loaded:', mapped.length, mapped);
    ensurePowerDrafts(mapped);
    syncPaginationTotal(mapped.length);
    updateStatistics(stats);
    
    // 加载充电桩配置
    await loadChargerConfigs(mapped.map(c => c.chargerId));
    
    if (showToast) {
      message.success('数据已刷新');
    }
  } catch (err) {
    console.error('Failed to refresh:', err);
    message.error(`刷新失败: ${errorMessage(err)}`);
  } finally {
    listLoading.value = false;
  }
};

const loadChargerConfigs = async (chargerIds: string[]) => {
  const configs: Record<string, ChargerConfig> = {};
  await Promise.all(
    chargerIds.map(async (id) => {
      try {
        const config = await invoke<ChargerConfig>('get_charger_config', { chargerId: id });
        configs[id] = config;
      } catch (err) {
        console.error(`Failed to load config for ${id}:`, err);
      }
    })
  );
  chargerConfigs.value = configs;
};

const startAutoRefresh = () => {
  if (refreshTimer) {
    clearInterval(refreshTimer);
  }
  if (autoRefresh.value) {
    refreshTimer = setInterval(() => {
      refreshDashboard();
    }, 5000);
  }
};

watch(autoRefresh, (value) => {
  if (value) {
    startAutoRefresh();
  } else if (refreshTimer) {
    clearInterval(refreshTimer);
    refreshTimer = undefined;
  }
});

onMounted(() => {
  refreshDashboard();
  startAutoRefresh();
});

onBeforeUnmount(() => {
  if (refreshTimer) {
    clearInterval(refreshTimer);
  }
});

const setActionLoading = (chargerId: string, action: string, value: boolean) => {
  const current = actionLoading.value[chargerId] ?? {};
  actionLoading.value = {
    ...actionLoading.value,
    [chargerId]: {
      ...current,
      [action]: value,
    },
  };
};

const isActionLoading = (chargerId: string, action: string) =>
  !!actionLoading.value[chargerId]?.[action];

const executeChargerAction = async (
  chargerId: string,
  action: string,
  task: () => Promise<unknown>,
  successMessage: string,
) => {
  setActionLoading(chargerId, action, true);
  try {
    await task();
    message.success(successMessage);
    await refreshDashboard();
  } catch (err) {
    message.error(`${successMessage}失败: ${errorMessage(err)}`);
  } finally {
    setActionLoading(chargerId, action, false);
  }
};

const startSingleCharger = (chargerId: string) =>
  executeChargerAction(
    chargerId,
    'start',
    () => invoke('start_charger', { chargerId }),
    '充电桩启动',
  );

const stopSingleCharger = (chargerId: string) =>
  executeChargerAction(
    chargerId,
    'stop',
    () => invoke('stop_charger', { chargerId }),
    '充电桩停止',
  );

const startChargingSession = (chargerId: string) =>
  executeChargerAction(
    chargerId,
    'startCharging',
    () => invoke('send_charger_command', { chargerId, command: { type: 'StartCharging' } }),
    '启动充电',
  );

const stopChargingSession = (chargerId: string) =>
  executeChargerAction(
    chargerId,
    'stopCharging',
    () => invoke('send_charger_command', { chargerId, command: { type: 'StopCharging' } }),
    '停止充电',
  );

const resetCharger = (chargerId: string) =>
  executeChargerAction(
    chargerId,
    'reset',
    () => invoke('send_charger_command', { chargerId, command: { type: 'Reset' } }),
    '重置充电桩',
  );

const startScript = (chargerId: string) =>
  executeChargerAction(
    chargerId,
    'startScript',
    () => invoke('start_charger_script', { chargerId }),
    '启动脚本',
  );

const stopScript = (chargerId: string) =>
  executeChargerAction(
    chargerId,
    'stopScript',
    () => invoke('stop_charger_script', { chargerId }),
    '停止脚本',
  );

const openScriptConfig = (charger: ChargerStateView) => {
  scriptModalVisible.value = true;
  scriptForm.value = {
    chargerId: charger.chargerId,
    name: charger.scriptName || `${charger.chargerId}-script`,
    autoStart: true,
    code: '',
    submitting: false,
    selectedPreset: '',
  };
};

interface PresetScript {
  key: string;
  name: string;
  description: string;
}

const presetScripts = ref<PresetScript[]>([]);

// 加载预设脚本列表
const loadPresetScriptList = async () => {
  try {
    presetScripts.value = await invoke<PresetScript[]>('get_preset_scripts');
  } catch (error) {
    console.error('加载预设脚本列表失败:', error);
  }
};

// 在组件挂载时加载预设脚本列表
onMounted(() => {
  loadPresetScriptList();
});

const loadPresetScript = async (presetKey: string) => {
  if (!presetKey) {
    scriptForm.value.code = '';
    scriptForm.value.name = `${scriptForm.value.chargerId}-script`;
    return;
  }

  try {
    const scriptCode = await invoke<string>('read_preset_script', { scriptKey: presetKey });
    const preset = presetScripts.value.find(p => p.key === presetKey);
    
    scriptForm.value.code = scriptCode;
    scriptForm.value.name = preset?.name || `${presetKey}.js`;
    message.success(`已加载脚本: ${preset?.name || presetKey}`);
  } catch (error) {
    message.error(`加载预设脚本失败: ${errorMessage(error)}`);
  }
};

const closeScriptModal = () => {
  scriptModalVisible.value = false;
};

const openConfigModal = async (charger: ChargerStateView) => {
  try {
    const config = await invoke<ChargerConfig>('get_charger_config', { chargerId: charger.chargerId });
    configForm.value = {
      ...config,
      submitting: false,
    };
    configModalVisible.value = true;
  } catch (err) {
    message.error(`加载配置失败: ${errorMessage(err)}`);
  }
};

const closeConfigModal = () => {
  configModalVisible.value = false;
};

const submitConfigForm = async () => {
  const form = configForm.value;
  if (!form.name.trim()) {
    message.warning('请填写充电桩名称');
    return;
  }
  if (!form.server_url.trim()) {
    message.warning('请填写服务器地址');
    return;
  }
  if (form.max_power <= 0) {
    message.warning('最大功率必须大于0');
    return;
  }

  configForm.value.submitting = true;
  try {
    const config: ChargerConfig = {
      id: form.id,
      name: form.name.trim(),
      protocol_type: form.protocol_type,
      server_url: form.server_url.trim(),
      max_power: form.max_power,
      script_path: form.script_path,
      enabled: form.enabled,
    };
    
    await invoke('update_charger_config', {
      chargerId: form.id,
      config,
    });
    
    message.success('配置已更新');
    configModalVisible.value = false;
    await refreshDashboard();
  } catch (err) {
    message.error(`配置更新失败: ${errorMessage(err)}`);
  } finally {
    configForm.value.submitting = false;
  }
};

const openAddChargerModal = () => {
  // 生成默认ID
  const existingIds = chargers.value.map(c => c.chargerId);
  let newId = 1;
  while (existingIds.includes(`CP${String(newId).padStart(6, '0')}`)) {
    newId++;
  }
  
  addChargerForm.value = {
    id: `CP${String(newId).padStart(6, '0')}`,
    name: `${newId}号充电桩`,
    protocol_type: 'OCPP',
    server_url: 'ws://localhost:8080/ocpp',
    max_power: 60,
    script_path: null,
    enabled: true,
    submitting: false,
  };
  addChargerModalVisible.value = true;
};

const closeAddChargerModal = () => {
  addChargerModalVisible.value = false;
};

const submitAddChargerForm = async () => {
  const form = addChargerForm.value;
  
  if (!form.id.trim()) {
    message.warning('请填写充电桩ID');
    return;
  }
  if (!form.name.trim()) {
    message.warning('请填写充电桩名称');
    return;
  }
  if (!form.server_url.trim()) {
    message.warning('请填写服务器地址');
    return;
  }
  if (form.max_power <= 0) {
    message.warning('最大功率必须大于0');
    return;
  }

  // 检查ID是否已存在
  if (chargers.value.some(c => c.chargerId === form.id.trim())) {
    message.error('充电桩ID已存在，请使用其他ID');
    return;
  }

  addChargerForm.value.submitting = true;
  try {
    const config: ChargerConfig = {
      id: form.id.trim(),
      name: form.name.trim(),
      protocol_type: form.protocol_type,
      server_url: form.server_url.trim(),
      max_power: form.max_power,
      script_path: form.script_path,
      enabled: form.enabled,
    };
    
    const newId = await invoke<string>('add_charger', { config });
    message.success(`充电桩 ${newId} 已添加`);
    addChargerModalVisible.value = false;
    
    // 如果启用了，尝试启动
    if (form.enabled) {
      try {
        await invoke('start_charger', { chargerId: newId });
        message.info(`充电桩 ${newId} 已启动`);
      } catch (err) {
        console.error('Failed to start charger:', err);
      }
    }
    
    await refreshDashboard();
  } catch (err) {
    message.error(`添加充电桩失败: ${errorMessage(err)}`);
  } finally {
    addChargerForm.value.submitting = false;
  }
};

const removeCharger = (chargerId: string) =>
  executeChargerAction(
    chargerId,
    'remove',
    async () => {
      await invoke('remove_charger', { chargerId });
      message.success(`充电桩 ${chargerId} 已删除`);
    },
    '删除充电桩',
  );

const submitScriptForm = async () => {
  const form = scriptForm.value;
  if (!form.chargerId) {
    message.error('未指定充电桩');
    return;
  }
  if (!form.name.trim()) {
    message.warning('请填写脚本名称');
    return;
  }
  if (!form.code.trim()) {
    message.warning('请填写脚本代码');
    return;
  }
  scriptForm.value.submitting = true;
  try {
    await invoke('set_charger_script', {
      chargerId: form.chargerId,
      payload: {
        name: form.name.trim(),
        code: form.code,
        autoStart: form.autoStart,
      },
    });
    message.success('脚本已配置');
    scriptModalVisible.value = false;
    await refreshDashboard();
  } catch (err) {
    message.error(`脚本配置失败: ${errorMessage(err)}`);
  } finally {
    scriptForm.value.submitting = false;
  }
};

const runGlobalAction = async (
  key: 'startAll' | 'stopAll' | 'reset',
  task: () => Promise<void>,
) => {
  if (globalLoading.value[key]) {
    return;
  }
  globalLoading.value = {
    ...globalLoading.value,
    [key]: true,
  };
  try {
    await task();
  } finally {
    globalLoading.value = {
      ...globalLoading.value,
      [key]: false,
    };
  }
};

const startSimulation = () =>
  runGlobalAction('startAll', async () => {
    const count = await invoke<number>('start_all_chargers');
    message.success(`已启动 ${count} 台充电桩`);
    await refreshDashboard();
  });

const stopSimulation = () =>
  runGlobalAction('stopAll', async () => {
    const count = await invoke<number>('stop_all_chargers');
    message.warning(`已停止 ${count} 台充电桩`);
    await refreshDashboard();
  });

const resetSimulation = () =>
  runGlobalAction('reset', async () => {
    await invoke<number>('stop_all_chargers');
    await invoke<number>('start_all_chargers');
    message.info('所有充电桩已重置');
    await refreshDashboard();
  });

const testScriptEngine = async () => {
  try {
    scriptTestLoading.value = true;
    message.loading({ content: '正在准备脚本测试...', key: 'scriptTest' });

    const scriptsToRun = [
      {
        id: 'basic_test',
        label: '基础功能回归脚本',
        code: testScriptSource,
      },
    ] as const;

    message.loading({ content: '正在并发执行脚本测试...', key: 'scriptTest' });

    const results = await Promise.all(
      scriptsToRun.map(async ({ id, label, code }) => {
        try {
          const result = await invoke<{ success: boolean; message: string }>('execute_script', {
            scriptId: id,
            scriptCode: code,
          });

          return {
            id,
            label,
            success: result?.success ?? true,
            message: result?.message ?? '脚本执行成功',
          };
        } catch (error) {
          const messageText = errorMessage(error);
          return {
            id,
            label,
            success: false,
            message: messageText,
          };
        }
      }),
    );

    const failed = results.filter((item) => !item.success);

    if (failed.length > 0) {
      message.error({
        content: `共有 ${failed.length} 个脚本失败，请查看详情。`,
        key: 'scriptTest',
        duration: 6,
      });
    } else {
      message.success({
        content: '全部脚本测试通过！请查看开发者工具的控制台输出。',
        key: 'scriptTest',
        duration: 3,
      });
    }

    console.table(
      results.map(({ id, label, success, message }) => ({
        脚本ID: id,
        描述: label,
        结果: success ? '✅ 成功' : '❌ 失败',
        说明: message,
      })),
    );

    return results;
  } catch (error) {
    message.error({ content: `测试失败: ${errorMessage(error)}`, key: 'scriptTest', duration: 5 });
    console.error('Script test error:', error);
    throw error;
  } finally {
    scriptTestLoading.value = false;
  }
};
</script>

<style scoped>
.app-container {
  width: 100%;
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  transition: background-color 0.3s ease;
}

.layout {
  flex: 1;
  min-height: 0;
}

/* ==================== 亮色模式样式 ==================== */
.light-mode .header {
  background: linear-gradient(135deg, #1890ff 0%, #096dd9 100%);
  padding: 0 16px;
  height: 48px;
  line-height: 48px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
}

.light-mode .header-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
  height: 100%;
}

.light-mode .logo {
  display: flex;
  align-items: center;
  gap: 12px;
  color: #fff;
}

.light-mode .logo-icon {
  font-size: 18px;
}

.light-mode .logo-text {
  font-size: 15px;
  font-weight: 600;
  letter-spacing: 0.5px;
}

.light-mode .header-actions :deep(.ant-btn) {
  color: #fff;
}

.light-mode .content {
  padding: 12px clamp(8px, 2vw, 16px);
  overflow-y: auto;
  overflow-x: hidden;
  background: linear-gradient(180deg, #f0f2f5 0%, #ffffff 100%);
  flex: 1;
  display: flex;
  justify-content: center;
}

/* ==================== 暗色模式样式 ==================== */
.dark-mode .header {
  background: linear-gradient(135deg, #141414 0%, #1f1f1f 100%);
  padding: 0 16px;
  height: 48px;
  line-height: 48px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.45);
  border-bottom: 1px solid #303030;
}

.dark-mode .header-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
  height: 100%;
}

.dark-mode .logo {
  display: flex;
  align-items: center;
  gap: 12px;
  color: rgba(255, 255, 255, 0.85);
}

.dark-mode .logo-icon {
  font-size: 18px;
  color: #1890ff;
}

.dark-mode .logo-text {
  font-size: 15px;
  font-weight: 600;
  letter-spacing: 0.5px;
}

.dark-mode .header-actions :deep(.ant-btn) {
  color: rgba(255, 255, 255, 0.85);
}

.dark-mode .content {
  padding: 12px clamp(8px, 2vw, 16px);
  overflow-y: auto;
  overflow-x: hidden;
  background: linear-gradient(180deg, #141414 0%, #1a1a1a 100%);
  flex: 1;
  display: flex;
  justify-content: center;
}

/* 暗色模式下的卡片样式 */
.dark-mode .welcome-card,
.dark-mode .action-card,
.dark-mode .charger-card,
.dark-mode .guide-card,
.dark-mode .stat-card {
  background: #1f1f1f;
  border: 1px solid #303030;
}

.dark-mode .welcome-card :deep(.ant-card-body),
.dark-mode .action-card :deep(.ant-card-body),
.dark-mode .charger-card :deep(.ant-card-body),
.dark-mode .guide-card :deep(.ant-card-body),
.dark-mode .stat-card :deep(.ant-card-body) {
  background: #1f1f1f;
}

/* 暗色模式下的表格样式 */
.dark-mode :deep(.ant-table) {
  background: #1f1f1f;
}

.dark-mode :deep(.ant-table-thead > tr > th) {
  background: linear-gradient(135deg, #1890ff 0%, #096dd9 100%) !important;
  color: #ffffff !important;
  border-bottom: 1px solid #303030;
}

.dark-mode :deep(.ant-table-tbody > tr) {
  background: #1f1f1f;
}

.dark-mode :deep(.ant-table-tbody > tr:hover > td) {
  background: #262626 !important;
}

.dark-mode :deep(.ant-table-tbody > tr > td) {
  border-bottom: 1px solid #303030;
}

/* 暗色模式下的展开行样式 */
.dark-mode .expanded-row {
  background: rgba(24, 144, 255, 0.08);
}

.dark-mode .expanded-row :deep(.ant-descriptions-item-label) {
  background: rgba(24, 144, 255, 0.12);
  color: rgba(255, 255, 255, 0.85);
}

.dark-mode .expanded-row :deep(.ant-descriptions-item-content) {
  background: #262626;
  color: rgba(255, 255, 255, 0.85);
}

/* 通用样式（不区分主题） */
.header {
  transition: all 0.3s ease;
}

.header-content {
  transition: all 0.3s ease;
}

.logo {
  transition: all 0.3s ease;
}

.content {
  transition: all 0.3s ease;
}

.content-inner {
  width: min(1800px, 100%);
  padding-bottom: 6px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.welcome-card {
  box-shadow: 0 1px 6px rgba(0, 0, 0, 0.06);
  border-radius: 7px;
}

.welcome-card :deep(.ant-card-body) {
  padding: 6px 8px;
}

.action-card {
  border-radius: 7px;
  box-shadow: 0 1px 6px rgba(0, 0, 0, 0.06);
}

.action-card :deep(.ant-card-body) {
  padding: 12px;
}

.charger-card {
  border-radius: 7px;
  box-shadow: 0 1px 6px rgba(0, 0, 0, 0.06);
}

.charger-card :deep(.ant-card-body) {
  padding: 12px;
}

.guide-card {
  border-radius: 7px;
  box-shadow: 0 1px 6px rgba(0, 0, 0, 0.06);
}

.guide-card :deep(.ant-card-body) {
  padding: 16px;
}

.charger-list-card :deep(.ant-card-body) {
  padding: 6px 8px 6px;
}

.welcome-title {
  font-size: 18px;
  font-weight: 700;
  margin: 0;
  background: linear-gradient(135deg, #1890ff 0%, #722ed1 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.welcome-subtitle {
  font-size: 12px;
  color: #8c8c8c;
  margin-top: 2px;
  margin-bottom: 0;
}

.stat-card {
  border-radius: 5px;
  transition: all 0.2s ease;
  min-height: 56px;
  font-size: 12px;
}

.stat-card:hover {
  transform: translateY(-1px);
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.10);
}

.charger-list-card {
  margin-top: 6px;
  border-radius: 5px;
}

.refresh-tip {
  font-size: 12px;
  color: var(--ant-text-color-secondary, #8c8c8c);
}

.empty-holder {
  padding: 48px 0;
  display: flex;
  justify-content: center;
}

.charger-table {
  margin-top: 8px;
}

.charger-cell {
  display: flex;
  align-items: center;
  gap: 10px;
}

.charger-cell-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.charger-id {
  font-weight: 600;
  font-size: 14px;
}

.charger-error {
  font-size: 12px;
  color: var(--ant-text-color-secondary, #8c8c8c);
}

.table-operations {
  display: flex;
  align-items: center;
  gap: 6px;
  flex-wrap: nowrap;
  justify-content: flex-start;
}

.table-operations .ant-btn-circle {
  width: 28px;
  height: 28px;
  min-width: 28px;
  padding: 0;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  font-size: 13px;
}

.operation-group {
  display: flex;
  align-items: center;
  gap: 6px;
  flex-wrap: wrap;
}

.operation-group :deep(.ant-btn) {
  min-width: 0;
  padding: 0 6px;
}

.operation-group :deep(.ant-btn.ant-btn-circle) {
  width: 28px;
  height: 28px;
  padding: 0;
}

.expanded-row {
  padding: 12px 16px;
  background: rgba(24, 144, 255, 0.04);
  border-radius: 6px;
}

.expanded-row :deep(.ant-descriptions-item-label) {
  font-weight: 600;
  background: rgba(24, 144, 255, 0.08);
}

.expanded-row :deep(.ant-descriptions-item-content) {
  background: rgba(255, 255, 255, 0.5);
}

.expanded-script-info {
  min-width: 220px;
  flex: 1;
}

.expanded-meta {
  font-size: 12px;
  color: var(--ant-text-color-secondary, #8c8c8c);
}

.script-message {
  font-size: 12px;
  color: var(--ant-text-color-secondary, #8c8c8c);
}

.script-action-bar {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  align-items: center;
}

.charger-table :deep(.ant-table-body) {
  max-height: none;
}

:deep(.ant-table) {
  background: transparent;
  font-size: 13px;
}

:deep(.ant-table-thead > tr > th) {
  background: linear-gradient(135deg, rgba(24, 144, 255, 0.95), rgba(9, 109, 217, 0.95)) !important;
  color: #ffffff !important;
  position: sticky;
  top: 0;
  z-index: 2;
  white-space: nowrap;
  padding: 10px 8px;
  font-weight: 600;
  font-size: 13px;
  letter-spacing: 0.3px;
  border-bottom: none;
}

:deep(.ant-table-thead > tr > th:first-child) {
  border-top-left-radius: 6px;
}

:deep(.ant-table-thead > tr > th:last-child) {
  border-top-right-radius: 6px;
}

:deep(.ant-table-thead > tr > th)::before {
  display: none;
}

:deep(.ant-table-tbody > tr > td) {
  vertical-align: middle;
  white-space: nowrap;
  padding: 8px 8px;
}

:deep(.ant-table-pagination) {
  margin: 12px 0 0;
}

.footer {
  text-align: center;
  padding: 8px 16px;
  transition: all 0.3s ease;
}

.light-mode .footer {
  background-color: #f0f2f5;
  color: rgba(0, 0, 0, 0.65);
  border-top: 1px solid #d9d9d9;
}

.dark-mode .footer {
  background-color: #141414;
  color: rgba(255, 255, 255, 0.65);
  border-top: 1px solid #303030;
}

.footer-content {
  display: flex;
  justify-content: space-between;
  max-width: 1800px;
  margin: 0 auto;
  font-size: 12px;
}

/* 暗色模式适配 */
:deep(.ant-layout) {
  background: transparent;
}

/* 高分辨率屏幕优化 */
@media (min-width: 1920px) {
  .welcome-title {
    font-size: 40px;
  }

  .welcome-subtitle {
    font-size: 18px;
  }

  .logo-text {
    font-size: 24px;
  }
}

/* 响应式布局 */
@media (max-width: 768px) {
  .header-content {
    flex-direction: column;
    height: auto;
    padding: 12px 0;
  }

  .content {
    padding: 12px;
  }

  .footer-content {
    flex-direction: column;
    gap: 8px;
  }

  .expanded-row {
    flex-direction: column;
  }
}
</style>
