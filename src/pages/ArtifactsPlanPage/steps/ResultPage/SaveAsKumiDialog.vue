<template>
    <el-dialog
        :visible="visible"
        @update:visible="$emit('update:visible', $event)"
        title="Save as Artifact Set"
    >
        <h3 class="config-title">Select Sets</h3>
        <div class="select-dir">
            <div
                v-for="dir in allDirs"
                :key="dir.id"
            >
                <el-radio
                    :label="dir.id"
                    v-model="dirId"
                >{{ dir.label }}</el-radio>
            </div>
        </div>

        <h3 class="config-title">Name</h3>
        <el-input v-model="kumiName"></el-input>

        <span slot="footer">
            <el-button
                @click="$emit('update:visible', false)"
            >No</el-button>
            <el-button
                type="primary"
                :disabled="!valid"
                @click="handleConfirm"
            >Yes</el-button>
        </span>
    </el-dialog>
</template>

<script>
export default {
    name: "SaveAsKumiDialog",
    props: ["visible"],
    data() {
        return {
            dirId: -1,
            kumiName: "New Name",
        }
    },
    created() {
        this.dirId = this.$store.getters["kumi/firstDirId"];
    },
    methods: {
        handleConfirm() {
            this.$emit("confirm", {
                dirId: this.dirId,
                name: this.kumiName,
            });
        }
    },
    computed: {
        // kumiByDir() {
        //     return this.$store.getters["kumi/kumiByDir"];
        // },

        allDirs() {
            return this.$store.getters["kumi/allDirs"];
        },

        treeData() {
            return this.$store.state.kumi["tree"].children;
        },

        valid() {
            return this.kumiName !== "";
        },
    }
}
</script>

<style lang="scss" scoped>
.select-dir {
    max-height: 300px;
    overflow: auto;
    margin-bottom: 16px;
}
</style>