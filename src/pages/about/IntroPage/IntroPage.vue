<template>
    <div>
        <el-breadcrumb>
            <el-breadcrumb-item>Main Page</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>
        <center>
        <p
            class="title"
        >
            <span class="mona">Genshin Artifacts Optimizer</span>
        </p>
        </center>
        

        <migrate-notification
            v-if="needMigrate"
            style="margin-bottom: 16px"
        ></migrate-notification>

        <el-alert
            class="hidden-sm-and-up"
            title="Use PC access to enable full functionality, currently only information pages can be viewed"
            type="error"
            style="margin-bottom: 16px"
            :closable="false"
        ></el-alert>
           <div>
                        <p style="font-size: 12px">Feddback</p>
                        <el-input v-model="feedback">
                            <template #append>
                                <el-button @click="handleClickSubmitFeedback">Submit</el-button>
                            </template>
                        </el-input>
        </div>

        <!--<div class="update-banner">
            <p style="font-size: 24px; margin: 0; color:white">V {{ version }}</p>
            <ul>
                <li
                    v-for="(item, index) in lastestVersion.changes"
                    :key="index"
                >{{ item }}</li>
            </ul>
        </div>-->

        <el-card class="mb16">
            <p class="big-card-title">Resources</p>
            <el-row :gutter="16">
                <el-col :xs="48" :sm="24">
                    <el-card
                        :body-style="{textAlign: 'center'}"
                        @click.native="newPage(links.frontendProject)"
                        class="item mb16"
                        shadow="hover"
                    >
                        <font-awesome-icon :icon="['fab', 'github']" class="icon"></font-awesome-icon>
                        <p class="item-title">Github - Project Source Code</p>
                    </el-card>
                </el-col>
            
            </el-row>
        </el-card>

        <el-row :gutter="48">
            <el-col :xs="24" :sm="12">
                <el-card class="mb16">
                    <p class="card-title">Contact the Developer</p>
                    <el-button @click="newPage(links.issue)" class="data-source">
                        Create a Github Issue
                        <font-awesome-icon :icon="['fab', 'github']"></font-awesome-icon>
                    </el-button>
                    <el-button @click="newPage(links.discord)"  class="data-source">
                        Discord Server
                        <font-awesome-icon icon="comment"></font-awesome-icon>
                    </el-button>
                </el-card>
            </el-col>
            <el-col :xs="24" :sm="12">
                <el-card class="mb16">
                    <p class="card-title">Useful Links </p>
                    <el-button
                        @click="$router.push('/help/basic')"
                        class="data-source"
                    >
                        Help
                        <i class="el-icon-question"></i>
                    </el-button>
                    <el-button
                        @click="$router.push('/help/export-tools')"
                        type="primary"
                        class="data-source"
                    >
                        Scaners
                        <i class="el-icon-aim"></i>
                    </el-button>
                    <el-button
                        @click="$router.push('/help/faq')"
                        class="data-source"
                    >
                        FAQ
                        <i class="el-icon-question"></i>
                    </el-button>
                     <el-button
                        @click="$router.push('/calculate')"
                        class="data-source"
                    >
                        Character Calculator
                        <i class="el-icon-cpu"></i>
                    </el-button>
                </el-card>
            </el-col>
            <el-col :xs="24" :sm="32">
                <el-card class="mb16">
                    <center>
                    <p class="card-title">Tutorial On How To Use Genshin Artifact Optimizer</p>
                    <iframe width="560" height="315" src="https://www.youtube.com/embed/KiBjW41g5vA" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>
                    </center>
                </el-card>
            </el-col>   
        </el-row>
    </div>
</template>

<script>
import changelogs from "@/changelog/changelog";
import links from "@const/links";
import { createFeedback } from "@/api/misc"

import MigrateNotification from "./MigrateNotification";
//import UseCaseItem from "./UseCaseItem";
// import NokNok from "./ad/NokNok";

export default {
    name: "IntroPage",
    components: {
        MigrateNotification,
        //UseCaseItem,
        // NokNok,
    },
      data() {
        return {
            feedback: ""
        }
    },
    created: function() {
        this.links = links;

        this.version = process.env.VERSION;
        this.webTitle = process.env.WEB_TITLE;
        this.needMigrate = process.env.NEED_MIGRATE;
        this.buildDate = process.env.BUILD_DATE;

        this.lastestVersion = changelogs[changelogs.length - 1];
        this.host = location.hostname;
    },
    methods: {
        navigateTo(des) {
            this.$router.replace(des);
        },
        newPage(des) {
            window.open(des, "_blank");
        },
                },
        handleClickSubmitFeedback() {
            if (this.feedback === "") {
                return
            }
            createFeedback(this.feedback)
            this.$message.success("Info Has been sent")
    }
}
</script>

<style lang="scss" scoped>
.build-info {
    font-size: 1rem;

}

.mb16 {
    margin-bottom: 16px;
}

.pay {
    display: flex;
    align-items: center;
    margin-top: 20px;

    span {
        width: 100px;
    }

    img {
        width: 96px;
        height: 96px;
        display: block;
    }
}

.update-banner {
    padding: 20px;
    color: #303133;
    background: rgba(82, 201, 238, 0.5);
    border: 1px solid #3cafe4;
    margin-bottom: 16px;
    border-radius: 3px;
}

.title {
    font-size: 3rem;
    margin: 0;
    margin-bottom: 24px;
    padding: 0;
}

.item {
    cursor: pointer;
}

.item-title {
    font-weight: bold;
    font-size: 1.2rem;
    padding: 0;
    margin: 16px 0 0 0;
}

.icon {
    font-size: 1.5rem;
}

.card-title {
    font-size: 1.2rem;
    margin: 0 0 16px 0;
}

.big-card-title {
    margin: 0;
    padding: 0 0 16px 0;
    font-size: 1.5rem;
}

.data-source {
    margin: 0;
    margin-right: 10px;
    margin-top: 10px;
}

.mona {
    background: #3cafe4;
    border-radius: 3px;
    color: #222933 ;
    padding: 0 8px;
}
</style>