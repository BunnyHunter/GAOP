<template>
  <div class="root">
    <el-menu default-active="intro" style="border: none" router :mode="mode">
      <el-menu-item index="/intro">
        <i class="el-icon-s-home"></i>
        <span>Main Page</span>
      </el-menu-item>

      <el-menu-item-group class="hidden-xs-only">
        <template #title> Storage </template>
        <el-menu-item index="/artifacts">
          <i class="el-icon-s-help"></i>
          Artifact Storage
        </el-menu-item>
        <el-menu-item index="/artifacts-kumi">
          <i class="el-icon-s-help"></i>
          Artifact Presets
        </el-menu-item>
        <el-menu-item index="/presets">
          <i class="el-icon-menu"></i>
          Character Presets
        </el-menu-item>
      </el-menu-item-group>

      <el-menu-item-group class="hidden-xs-only">
        <template #title> Calculations </template>
        <el-menu-item index="/calculate">
          <i class="el-icon-cpu"></i>
          Character Calculator
        </el-menu-item>
        <el-menu-item index="/team-optimization">
          <i class="el-icon-s-opportunity"></i>
          Team Calculator
        </el-menu-item>
        <el-menu-item index="/potential">
          <i class="el-icon-magic-stick"></i>
          Artifacts Potential
        </el-menu-item>
        <el-menu-item index="/mona-db">
        <i class="el-icon-s-data"></i>
          Database
        </el-menu-item>
        <!--                <el-menu-item index="/artifacts-statistics">-->
        <!--                    <i class="el-icon-s-data"></i>-->
        <!--                    Artifact Statistics-->
        <!--                </el-menu-item>-->
      </el-menu-item-group>

      <el-menu-item-group>
        <template #title> About this site </template>
        <el-submenu index="help">
          <template slot="title">
            <i class="el-icon-question"></i>
            Help
          </template>
          <el-menu-item index="/help/basic">
            <i class="el-icon-question"></i>
            Basic Usage
          </el-menu-item>
          <el-menu-item index="/help/export-tools">
            <i class="el-icon-aim"></i>
            Export Tools / Scanners
          </el-menu-item>
          <el-menu-item index="/help/faq">
            <i class="el-icon-question"></i>
            FAQ
          </el-menu-item>
          <el-menu-item index="/help/target-func-explanation">
            <i class="el-icon-question"></i>
            Target Function Parameters Description
          </el-menu-item>
        </el-submenu>
        <!--<el-submenu index="alg">
          <template slot="title">
            <i class="el-icon-ice-cream"></i>
            Algorithm Description
          </template>
          <el-menu-item index="/algorithm-target">
            <i class="el-icon-chicken"></i>
            Target function
          </el-menu-item>
          <el-menu-item index="/algorithm-potential">
            <i class="el-icon-sugar"></i>
            Potential function
          </el-menu-item>
        </el-submenu>-->

        <!--<el-menu-item index="/changelog">
          <i class="el-icon-date"></i>
          Update Log
        </el-menu-item>-->

        <!--<el-menu-item index="/tomodachi">
          <i class="el-icon-link"></i>
          Links
        </el-menu-item>-->
      </el-menu-item-group>

    <!--<el-menu-item-group>
                <template #title>
                    Help
                </template>
                <el-menu-item index="use-case">
                    <i class="el-icon-question"></i>
                    Complete Usage Example
                </el-menu-item>
                <el-menu-item index="alg">
                    <i class="el-icon-s-opportunity"></i>
                    Algorithms
                </el-menu-item>
            </el-menu-item-group> -->
    </el-menu>
  </div>
</template>

<script>
export default {
  name: "SideBar",
  props: {
    items: {
      type: Array,
      default: () => [],
    },
    mode: {
      type: String,
      default: "vertical",
    },
  },
};
</script>

<style scoped>
.top {
  height: 220px;
  background: #409eff;
  display: flex;
  align-items: center;
  justify-content: center;
}

/* .root {
    background: #3cafe4;
} */

.item {
  padding: 0 32px;
}
</style>
