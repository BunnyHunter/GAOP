<template>
    <div :class="$style.client">
        <div class="client-content">
            <div v-if="loading">
                <div :class="$style.loader">
                    <div class="loader-animation">
                        <span class="cssload-loader"><span class="cssload-loader-inner"></span></span>
                    </div>
                    <div class="loader-text">Connecting to external plugin</div>
                </div>
            </div>
            <div v-if="notFound && !loading" class="not-found">
                <div class="title">
                    {{ needUpdate ? 'Your helper plugin version is too old' : 'Auxiliary plug-ins are required for automatic operation' }}
                    <small v-if="needUpdate">(v{{ version }})</small>
                </div>
                <div class="content">
                    Due to browser limitations, automated actions cannot be done on the web page
                    <a class="dlink" href="https://cocogoat.work/extra/client" target="_blank">
                        Click here to download the auxiliary plugin<small>(v{{ cVersion }} 300kB)</small>
                    </a>
                    <div class="absolute-area">
                        <el-button class="start-btn start-gray" @click="enable(false)">
                            <div class="l">
                                <i class="el-icon-check"></i>
                            </div>
                            <div class="m">
                                <div class="t">
                                    {{ needUpdate ? 'I have updated and re-run the client' : 'I have downloaded and run the client' }}
                                </div>
                                <div class="d">Remember to agree to the permission application for controlling the keyboard and mouse</div>
                            </div>
                            <div class="r">
                                <i class="el-icon-arrow-right"></i>
                            </div>
                        </el-button>
                    </div>
                </div>
            </div>
            <div v-if="!notFound && (denied || gameNotFound) && !loading" class="not-found">
                <div class="title">{{ denied ? 'You have denied permission request for helper plugin' : 'No available Genshin Impact windows found' }}</div>
                <div class="content">
                    <div v-if="denied">
                        Due to browser limitations, automated actions cannot be done on the web page<br />
                        Simulating keyboard and mouse operations requires you to run the auxiliary plug-in and agree to the permission application
                    </div>
                    <div v-else>
                        Automatic operation only supports windows PC client<br />
                        Please recheck if Genshin Impact is enabled
                    </div>
                    <div class="absolute-area">
                        <el-button class="start-btn start-gray" @click="enable(false)">
                            <div class="l">
                                <i class="el-icon-check"></i>
                            </div>
                            <div class="m">
                                <div class="t">Click to try again</div>
                                <div class="d">Remember to agree to the permission application for controlling the keyboard and mouse</div>
                            </div>
                            <div class="r">
                                <i class="el-icon-arrow-right"></i>
                            </div>
                        </el-button>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script>
import { ref, defineComponent, toRef, onMounted } from '@vue/composition-api';
// format: x.x.x
export function versionCompare(a, b) {
    const aParts = a.split('.');
    const bParts = b.split('.');
    const len = Math.max(aParts.length, bParts.length);
    for (let i = 0; i < len; i++) {
        const aPart = parseInt(aParts[i], 10);
        const bPart = parseInt(bParts[i], 10);
        if (aPart === bPart) {
            continue;
        }
        if (aPart === undefined) {
            return -1;
        }
        if (bPart === undefined) {
            return 1;
        }
        return aPart - bPart > 0 ? 1 : -1;
    }
    return 0;
}
export default defineComponent({
    props: {
        control: {
            type: Object,
            required: true,
        },
    },
    emits: ['done'],
    setup(props, { emit }) {
        const w = toRef(props, 'control');
        const loading = ref(true);
        const notFound = ref(false);
        const denied = ref(false);
        const gameNotFound = ref(false);
        const needUpdate = ref(false);
        const version = ref('');
        const enable = async (force = false) => {
            if (loading.value && !force) return;
            loading.value = true;
            denied.value = false;
            const alive = await w.value.check();
            if (!alive) {
                notFound.value = true;
                loading.value = false;
                needUpdate.value = false;
                return;
            } else if (versionCompare(w.value.version, '1.2.0') < 0) {
                needUpdate.value = true;
                notFound.value = true;
                loading.value = false;
                version.value = w.value.version;
                return;
            } else {
                needUpdate.value = false;
                try {
                    loading.value = true;
                    const authorized = await w.value.authorize();
                    if (authorized) {
                        let windows = await w.value.listWindows();
                        windows = windows.filter(
                            (w) => w.title.includes('原神') || w.title.toLowerCase().includes('genshin'),
                        );
                        if (windows.length === 0) {
                            notFound.value = false;
                            loading.value = false;
                            gameNotFound.value = true;
                            return;
                        }
                        emit('done', windows[0].hWnd);
                    } else {
                        denied.value = true;
                    }
                    notFound.value = false;
                } catch (e) {
                    notFound.value = true;
                    console.log(e);
                }
            }
            loading.value = false;
        };
        onMounted(() => enable(true));
        return {
            loading,
            notFound,
            enable,
            denied,
            gameNotFound,
            needUpdate,
            version,
            cVersion: '1.2.0',
        };
    },
});
</script>

<style lang="scss" module>
.client {
    padding-top: 10vh;
    :global {
        .client-content {
            height: 250px;
            position: relative;
            .absolute-area {
                position: absolute;
                left: 0;
                right: 0;
                bottom: 0;
            }
        }
        .not-found {
            .title {
                font-size: 23px;
                margin-bottom: 10px;
            }
            a {
                color: #555;
                text-decoration: none;
                &.dlink {
                    margin-top: 8px;
                    display: block;
                    color: #0068b7;
                }
            }
        }
    }
}
.loader {
    width: 200px;
    color: #666;
    text-align: center;
    font-size: 14px;
    margin: 0 auto;
    padding-top: 50px;
    :global {
        .loader-text {
            padding-top: 15px;
        }
        .cssload-loader {
            display: block;
            margin: 0 auto;
            width: 30px;
            height: 30px;
            position: relative;
            border: 3px solid #333;
            &:local {
                animation: scanner-cssload-loader 2.3s infinite ease;
            }
        }
        .cssload-loader-inner {
            vertical-align: top;
            display: inline-block;
            width: 100%;
            background-color: #333;
            &:local {
                animation: scanner-cssload-loader-inner 2.3s infinite ease-in;
            }
        }
        .loader-progress {
            width: 170px;
            height: 2px;
            background: #ddd;
            margin: 0 auto;
            margin-top: 20px;
            position: relative;
            .loader-progress-bar {
                width: 0;
                height: 100%;
                background: #777;
                position: absolute;
                left: 0;
                top: 0;
            }
        }
        .loader-progress-text {
            font-size: 12px;
            position: relative;
            top: -12px;
            background: #fff;
            display: inline-block;
            padding: 0 4px;
        }
    }
}
@keyframes scanner-cssload-loader {
    0% {
        transform: rotate(0deg);
    }
    25% {
        transform: rotate(180deg);
    }
    50% {
        transform: rotate(180deg);
    }
    75% {
        transform: rotate(360deg);
    }
    100% {
        transform: rotate(360deg);
    }
}
@keyframes scanner-cssload-loader-inner {
    0% {
        height: 0%;
    }
    25% {
        height: 0%;
    }
    50% {
        height: 100%;
    }
    75% {
        height: 100%;
    }
    100% {
        height: 0%;
    }
}
</style>