<template>
    <el-select
        :value="value"
        @input="$emit('input', $event)"
        placeholder="Character"
        size="small"
    >
        <el-option-group
            v-for="(group, elementName) in characterByElement"
            :key="elementName"
            :label="element2Chs(elementName)"
        >
            <el-option
                v-for="character in group"
                :key="character.name"
                :label="character.chs"
                :value="character.name"
            >
                <div class="option-item flex-row">
                    <img :src="character.avatar">
                    <span :style="{ color: getColor(character.star) }">{{ character.chs }}</span>
                </div>
            </el-option>
        </el-option-group>
    </el-select>
</template>

<script>
import { characterByElement } from "@character";
import qualityColors from "@const/quality_colors";

export default {
    name: "SelectCharacter",
    props: ["value"],
    created() {
        this.characterByElement = characterByElement;
    },
    methods: {
        element2Chs(element) {
            switch(element) {
                case "Pyro":
                    return "Pyro";
                case "Cryo":
                    return "Cryo";
                case "Dendro":
                    return "Dendro";
                case "Electro":
                    return "Electro";
                case "Anemo":
                    return "Anemo";
                case "Geo":
                    return "Geo";
                case "Hydro":
                    return "Hydro";
            }
        },

        getColor(star) {
            return qualityColors[star - 1];
        }
    }
}
</script>

<style lang="scss" scoped>
.option-item {
    img {
        width: 28px;
        height: 28px;
        margin-right: 8px;
        border-radius: 50%;
    }
}
</style>