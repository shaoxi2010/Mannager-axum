<script setup>
import axios from 'axios';
import { ref } from 'vue'
const ipaddr = ref('')
const passWord = ref('')
const deviceName = ref('')

const click = () => {
    axios.post("/devices",
        {
            name: deviceName.value,
            ipaddr: ipaddr.value,
            password: passWord.value,
        },
        { headers: { 'Content-Type': 'application/json' } }
        ).then(function() {
            alert("注册设备成功")
        }).catch(function() {
            alert("注册设备失败")
        })
}
</script>

<template>
    <div class="container">
        <h1 class="title">输入设备的名称:</h1>
        <el-input type="text" v-model="deviceName" class="input"></el-input>
        <h1 class="title">输入设备的IP地址:</h1>
        <el-input type="text" v-model="ipaddr" class="input"></el-input>
        <h1 class="title">输入设备的密码:</h1>
        <el-input type="text" v-model="passWord" class="input"></el-input>
        <el-button class="btn" @click="click">执行添加</el-button>
    </div>
</template>

<style scoped>
.container {
    margin: 0 auto;
    margin-top: 70px;
    text-align: center;
    width: 300px;
}
.title {
    text-align: left;
    font-size: 14px;
}
.input {
    width: 98%;
}

.btn {
    margin-top: 15px;
    width: 98%;
}
</style>