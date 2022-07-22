<template>
    <el-dialog
        title="Scanning Artifacts with YAS WebUI"
        :visible="visible"
        width="80%"
        @update:visible="$emit('update:visible', $event)"
    >
        <div :class="$style.webcapturer">
            <client-comp v-if="visible && !connected" :control="control" @done="onConnected" />
            <div v-if="connected" :class="$style.uimain">
                <div v-if="step === 2">
                    <el-progress type="circle" :percentage="statusProgress"></el-progress>
                    <br />
                    <div class="progress-text">
                        {{ statusText }}
                    </div>
                </div>
                <div v-if="step === 3">
                    <el-form>
                        <el-form-item label="Minimum Level">
                            <el-input-number v-model="form['min-level']" placeholder="Please enter a minimum level" />
                        </el-form-item>
                        <el-form-item label="Minimum Star Rating">
                            <el-input-number v-model="form['min-star']" placeholder="Please enter a minimum star rating" />
                        </el-form-item>
                        <el-form-item label="Number of Scans">
                            <el-input-number v-model="form['number']" placeholder="Please enter the number of scans" />
                            <br />
                            <small>0 is unlimited</small>
                        </el-form-item>
                        <div>
                            <el-checkbox v-model="importDeleteUnseen" style="margin-top: 12px"
                                >Delete non-existent artifacts</el-checkbox
                            >
                        </div>
                        <el-button class="start-btn start-gray" @click="startScan">
                            <div class="l">
                                <i class="el-icon-check"></i>
                            </div>
                            <div class="m">
                                <div class="t">Start Scanning</div>
                                <div class="d">YAS will run automatically after clicked...</div>
                            </div>
                            <div class="r">
                                <i class="el-icon-arrow-right"></i>
                            </div>
                        </el-button>
                    </el-form>
                </div>
                <div v-if="step === 4" style="width: 100%">
                    <textarea ref="outputArea" class="output" :value="output.join('\n')" readonly />
                </div>
            </div>
        </div>
    </el-dialog>
</template>

<script>
import ClientComp from './Client';
import { CocogoatWebControl } from './webcontrol';
import { importMonaJson } from '@util/artifacts';
const delay = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
export default {
    components: {
        ClientComp,
    },
    props: {
        visible: {
            type: Boolean,
        },
    },
    data() {
        return {
            control: new CocogoatWebControl(),
            connected: false,
            step: 1,
            statusText: 'Checking for Updates...',
            statusProgress: 0,
            form: {
                'min-level': 0,
                'min-star': 5,
                number: 0,
            },
            output: [],
            importDeleteUnseen: false,
            hwnd: -1,
        };
    },
    watch: {
        visible(val) {
            if (val) {
                this.connected = !!this.control.ws;
                this.step = this.connected ? 2 : 1;
            }
        },
        step(val) {
            if (val === 2) {
                this.loopForUpdate();
            }
        },
        output: {
            async handler(val) {
                await this.$nextTick();
                this.$refs.outputArea.scrollTop = this.$refs.outputArea.scrollHeight;
            },
            deep: true,
        },
    },
    methods: {
        onConnected(hwnd) {
            this.connected = true;
            this.step = 2;
            this.hwnd = hwnd;
        },
        async startScan() {
            this.step++;
            this.control.ev.removeAllListeners('yas');
            this.control.ev.removeAllListeners('yas-output');
            this.output = [];
            this.control.ev.on('yas', async (data) => {
                this.output.push(`[YAS STAT] ${data}`);
                if (data === 'exit') {
                    this.output.push(`[YAS PROG] Fetching Artifact Data...`);
                    const res = await this.control.client.get('/api/yas');
                    if (res instanceof Error) {
                        this.output.push(`[YAS ERR!] Failed to get the artifact data, the error details are as follows: \n${res.message}`);
                    } else {
                        const yasJson = res.data;
                        try {
                            importMonaJson(yasJson, this.importDeleteUnseen);
                            this.output.push(`[YAS DONE] The artifact scanning is completed, you can close this window and auxiliary plug-ins.`);
                        } catch (e) {
                            this.output.push(`[YAS ERR!] Failed to import artifact data, the error details are as follows: \n${res.message}`);
                        }
                    }
                }
            });
            this.control.ev.on('yas-output', (data) => {
                this.output.push(data);
            });
            if (this.hwnd > -1) {
                await this.control.activateWindow(this.hwnd);
            }
            this.control.wsInvoke('POST', '/api/yas', '', {
                argv: Object.keys(this.form).reduce((acc, key) => {
                    if (this.form[key] === 0) return acc;
                    acc += `--${key} ${this.form[key]} `;
                    return acc;
                }, ''),
            });
        },
        async loopForUpdate() {
            const msglist = {
                started: 'Checking for Updates...[0]',
                prechecking: 'Checking for Updates...[1]',
                prehashing: 'Checking for Updates...[2]',
                prechecksum: 'Checking for Updates...[3]',
                downloading: 'Downloading Update...',
                checksum: 'Checking for Updates...',
                done: 'Update Completed',
                noupdate: 'No Update',
            };
            const res = await this.control.wsInvoke('POST', '/api/upgrade/yas');
            if (res.status === 201) {
                while (1) {
                    const {
                        body: { total, downloaded, msg },
                    } = await this.control.wsInvoke('GET', '/api/upgrade/yas');
                    if (msg === 'done' || msg === 'noupdate') {
                        this.step++;
                        this.statusText = msglist[msg];
                        this.statusProgress = 100;
                        break;
                    } else {
                        const percent = Math.floor((downloaded / total) * 100) || 0;
                        this.statusProgress = percent;
                        if (msglist[msg]) {
                            this.statusText = msglist[msg] + '  ' + percent + '%';
                        } else {
                            this.statusText = 'ERROR: ' + msg;
                            break;
                        }
                    }
                    await delay(80);
                }
            }
        },
    },
};
</script>

<style lang="scss" module>
.webcapturer {
    text-align: center;
    :global {
        .output {
            width: 100%;
            height: calc(240px + 10vh);
            box-sizing: border-box;
            font-family: Consolas, monospace;
            color: #555;
        }
        .start-btn {
            margin-top: 10px;
            height: 60px;
            text-align: left;
            font-size: 16px;
            transition: all 0.3s;
            width: 490px;
            max-width: 95%;
            box-sizing: border-box;
            &:hover {
                transform: translateY(-5px);
            }
            &.start-gray {
                --el-button-hover-text-color: #333;
                --el-button-hover-border-color: #aaa;
                --el-button-hover-bg-color: #fafafa;
            }
            & > span {
                display: flex;
                width: 100%;
                height: 100%;
                justify-content: flex-start;
            }
            .r {
                opacity: 0.8;
                i {
                    font-size: 30px;
                }
            }
            .m {
                flex-grow: 1;
            }
            .l i {
                width: 40px;
                height: 24px;
                padding-right: 10px;
                font-size: 30px;
            }
            .d {
                font-size: 12px;
                margin-top: 3px;
                opacity: 0.8;
            }
        }
    }
}
.uimain {
    min-height: calc(10vh + 250px);
    display: flex;
    align-items: center;
    justify-content: center;
}
</style>