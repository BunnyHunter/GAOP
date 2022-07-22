<template>
    <div>
        <el-tabs v-model="activeName" type="card" :stretch="true">
            <el-tab-pane
                v-for="artType in artifactsType"
                :key="artType.name"
                class="panel"
                :name="artType.name"
            >
                <div slot="label" class="flex-row">
                    <img :src="artType.iconURL">
                    <span>{{ artType.chs }}</span>
                </div>
                <artifact-display
                    class="artifact-panel"
                    v-for="(item, index) in filteredArtifacts[artType.name]"
                    :key="artType.name + item.id"
                    :item="item"
                    :selectable="true"
                    @click="handleClick(artType.name, index)"
                ></artifact-display>
            </el-tab-pane>
        </el-tabs>
    </div>
</template>

<script>
import ArtifactDisplay from "@c/ArtifactDisplay";

import { artifactsIcon } from "@asset/artifacts";

export default {
    name: "SelectArtifact",
    components: {
        ArtifactDisplay,
    },
    data: function () {
        return {
            activeName: "flower",
        }
    },
    created: function () {
        this.artifactsType = [
            { name: "flower", chs: "Flower of Life", iconURL: artifactsIcon["flower"] },
            { name: "feather", chs: "Plume of Death", iconURL: artifactsIcon["feather"] },
            { name: "sand", chs: "Sands of Eon", iconURL: artifactsIcon["sand"] },
            { name: "cup", chs: "Goblet", iconURL: artifactsIcon["cup"] },
            { name: "head", chs: "Circlet", iconURL: artifactsIcon["head"] }
        ];
    },
    methods: {
        handleClick(position, index) {
            let art = this.filteredArtifacts[position][index];
            this.$emit("select", art);
        }
    },
    computed: {
        filteredArtifacts() {
            let allArtifacts = this.$store.getters["artifacts/allArtifacts"];
            let fil = art => (art.star || 5) === 5;
            let temp = {};
            for (let i in allArtifacts) {
                temp[i] = allArtifacts[i].filter(fil);
            }

            return temp;
        }
    }
}
</script>

<style scoped>
.panel {
    display: flex;
    flex-wrap: wrap;

}

.artifact-panel {
    margin: 8px;
}
</style>