<template>
    <div>
        <h3 class="class">Basic Atributes</h3>
        <attribute-item title="HP" :composition="attribute.hp"></attribute-item>
        <attribute-item title="ATK" :composition="attribute.atk"></attribute-item>
        <attribute-item title="DEF" :composition="attribute.def"></attribute-item>
        
        <attribute-item title="Elemental Mastery" :composition="attribute.elemental_mastery"></attribute-item>

        <h3 class="class">Advanced Atributes</h3>
        <attribute-item title="Crit Rate" :composition="attribute.critical" :percentage="true"></attribute-item>
        <attribute-item title="Crit Damage" :composition="attribute.critical_damage" :percentage="true"></attribute-item>

        <attribute-item title="Healing Bonus" :composition="attribute.healing_bonus" :percentage="true"></attribute-item>
        <attribute-item title="Energy Recharge" :composition="attribute.recharge" :percentage="true"></attribute-item>
         <attribute-item title="Shield Strength" :composition="attribute.shield_strength" :percentage="true"></attribute-item>

        <h3 class="class">Elemental Bonuses</h3>
        <attribute-item title="Pyro Element Damage Bonus" :composition="attribute.bonus_pyro" :percentage="true"></attribute-item>
        <attribute-item title="Hydro Element Damage Bonus" :composition="attribute.bonus_hydro" :percentage="true"></attribute-item>
        <attribute-item title="Dendro Element Damage Bonus" :composition="attribute.bonus_dendro" :percentage="true"></attribute-item>
        <attribute-item title="Electro Element Damage Bonus" :composition="attribute.bonus_electro" :percentage="true"></attribute-item>
        <attribute-item title="Anemo Element Damage Bonus" :composition="attribute.bonus_anemo" :percentage="true"></attribute-item>
        <attribute-item title="Cryo Element Damage Bonus" :composition="attribute.bonus_cryo" :percentage="true"></attribute-item>
        <attribute-item title="Geo Element Damage Bonus" :composition="attribute.bonus_geo" :percentage="true"></attribute-item>
        <attribute-item title="Physical Damage Bonus" :composition="attribute.bonus_physical" :percentage="true"></attribute-item>

<!--        <h3 class="class">元素</h3>-->
        <!-- <div class="item">
            <span class="title">元素精通</span>
            <span class="value">{{ attribute.elementalMastery }}</span>
        </div>
        <div class="item">
            <span class="title">元素充能效率</span>
            <span class="value">{{ mul100("recharge") }}</span>
        </div>
        <div class="item">
            <span class="title">超载增强</span>
            <span class="value">{{ mul100("overloadEnhance") }}</span>
        </div>
        <div class="item">
            <span class="title">燃烧增强</span>
            <span class="value">{{ mul100("burningEnhance") }}</span>
        </div>
        <div class="item">
            <span class="title">感电增强</span>
            <span class="value">{{ mul100("electroEnhance") }}</span>
        </div>
        <div class="item">
            <span class="title">超导增强</span>
            <span class="value">{{ mul100("superconductEnhance") }}</span>
        </div>
        <div class="item">
            <span class="title">扩散（Electro）增强</span>
            <span class="value">{{ mul100("swirlThunderEnhance") }}</span>
        </div>
        <div class="item">
            <span class="title">扩散（Pyro）增强</span>
            <span class="value">{{ mul100("swirlFireEnhance") }}</span>
        </div>
        <div class="item">
            <span class="title">扩散（Hydro）增强</span>
            <span class="value">{{ mul100("swirlWaterEnhance") }}</span>
        </div>
        <div class="item">
            <span class="title">扩散（Cryo）增强</span>
            <span class="value">{{ mul100("swirlIceEnhance") }}</span>
        </div>
        <div class="item">
            <span class="title">扩散增强</span>
            <span class="value">{{ mul100("swirlEnhance") }}</span>
        </div>
        <div class="item">
            <span class="title">Vaporize增强</span>
            <span class="value">{{ mul100("vaporizeEnhance") }}</span>
        </div>
        <div class="item">
            <span class="title">Melt增强</span>
            <span class="value">{{ mul100("meltEnhance") }}</span>
        </div> -->
    </div>
</template>

<script>
import AttributeItem from "./AttributeItem"

export default {
    name: "AttributePanel",
    components: {
        AttributeItem
    },
    props: {
        attribute: {
            type: Object,
        },
    },
    methods: {
        mul100(field) {
            return (this.attribute[field] * 100).toFixed(1) + "%";
        }
    },
    computed: {
        attack() {
            let p = this.attribute;
            return p.attackStatic + p.attackBasic + p.attackPercentage;
        },

        defend() {
            let p = this.attribute;
            return p.defendStatic + p.defendBasic + p.defendPercentage;
        },

        life() {
            let p = this.attribute;
            return p.lifeStatic + p.lifeBasic + p.lifePercentage;
        }
    }
}
</script>

<style lang="scss" scoped>
*{

    background-color: #222933;
}
.attribute {
    margin-bottom: 8px;
}

.class {
//background:rgb(0, 173, 255);
    // padding: 0px 16px;
    // display: inline-block;
    /* color: white; */
    border-radius: 3px;
    font-size: 14px;
    border-bottom: 5px solid rgb(0, 173, 255);;
    color: #ffff;
}
</style>