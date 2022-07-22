<template>
    <div>
        <el-alert
            title="Warning: Adding too many stats will make the calculation slow (exponentially slow), so please carefully consider which are valid terms, and the number of recommended is no more than 6"
            :closable="false"
            type="warning"
            style="margin-bottom: 16px"
        ></el-alert>

        <div class="config-item">
            <h3 class="config-title">Stats and their Weights</h3>
            <div
                v-for="tag in tags"
                :key="tag.name"
                class="tag-item"
            >
                <el-switch
                    :active-text="tag.chs"
                    v-model="weight[tag.name][0]"
                    class="tag-switch"
                ></el-switch>
                <el-input
                    v-model="weight[tag.name][1]"
                    size="small"
                ></el-input>
            </div>
        </div>
    </div>
</template>

<script>
import { artifactsSecondaryTag } from "@asset/artifacts";
import { artifactTags } from "@const/artifact";

export default {
    name: "Ratio.ptcfg",
    created() {
        let temp = {};
        for (let i of artifactsSecondaryTag) {
            temp[i] = artifactTags[i];
        }

        this.tags = temp;
    },
    data() {
        let weight = {};
        for (let i of artifactsSecondaryTag) {
            weight[i] = [false, 0];
        }
        weight["attackPercentage"] = [true, 1];
        weight["critical"] = [true, 1];
        weight["criticalDamage"] = [true, 1];

        return {
            weight,
            // validTag: ["attackPercentage", "critical", "criticalDamage"],
        }
    },
}
</script>

<style lang="scss" scoped>
.tag-item {
    margin-bottom: 16px;
    padding-bottom: 16px;
    border-bottom: 1px solid #eee;

    &:last-of-type {
        margin-bottom: 0;
        padding-bottom: 0;
        border: none;
    }

    &:first-of-type {
        margin-top: 16px;
    }

    .tag-switch {
        margin-bottom: 8px;
    }
}
</style>