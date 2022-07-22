<template>
    <div>
        <el-breadcrumb>
            <el-breadcrumb-item>Help</el-breadcrumb-item>
            <el-breadcrumb-item>Export Tools Summary</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>

        <!--<el-card class="item">
            <div slot="header">
                {{ getIndex() }}. Yas
            </div>

            <p>Exporter from the original author of Mona-Urenai : Yet Another Scanner, state-of-the-art (speed on) exporter</p>

            <div class="footer">
                <a target="_blank" href="https://github.com/wormtql/yas"
                    class="el-button no-deco el-button--primary"
                >
                    <i class="el-icon-connection"></i>
                    Project Address - Github
                </a>
            </div>
        </el-card>-->

        <el-card class="item">
            <div slot="header">
                1. Amenoma
            </div>

            <p>Genshin Artifact Scanner</p>

            <div class="footer">
                <a target="_blank" href="https://github.com/daydreaming666/Amenoma"
                    class="el-button no-deco el-button--primary"
                >
                    <i class="el-icon-connection"></i>
                    Project Address - Github
                </a>
            </div>
        </el-card>

        <el-card class="item">
            <div slot="header">
                2. Genshin Artifact Scanner
            </div>

            <p>Local recognition, basically fully automatic, no UI mode, speed roughly 100-200 per minute</p>

            <div class="footer">
                <a target="_blank" href="https://github.com/ProblemFactory/GenshinArtScanner" class="el-button no-deco">
                    <i class="el-icon-connection"></i>
                    Project Address - Github
                </a>
                <a
                    target="_blank"
                    href="https://bbs.nga.cn/read.php?tid=26425531&_ff=650&rand=246"
                    class="el-button no-deco"
                >
                    <i class="el-icon-connection"></i>
                    Instructions for use
                </a>
            </div>
        </el-card>

        <!--<el-card class="item">
            <div slot="header">3. Cocogoat (Chinese)</div>

            <p>Local recognition, basically fully automatic, with UI</p>

            <div class="footer">
                <a target="_blank" href="https://77.cocogoat.work/v1/ascension/" class="el-button el-button--primary no-deco">
                    <i class="el-icon-connection"></i>
                    Download Address
                </a>
                <a target="_blank" href="https://github.com/YuehaiTeam/cocogoat" class="el-button no-deco">
                    <i class="el-icon-connection"></i>
                    Project Address - Github
                </a>
            </div>
        </el-card>-->
    </div>
</template>

<script>
let iter = 0;

export default {
    name: "ExportToolPage",
    methods: {
        getIndex() {
            iter += 1;
            return iter;
        }
    }
}
</script>

<style scoped>
.item {
    margin-bottom: 16px;
}
</style>