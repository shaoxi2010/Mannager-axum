<script setup>
import { ref } from 'vue'

const activeIndex = ref(1)

</script>

<template>
  <el-header>
    <el-menu mode="horizontal" background-color="#545c64" :default-active="activeIndex">
      <router-link to="/">
        <el-menu-item index="1" >在线设备</el-menu-item>
      </router-link>
      <router-link to="/add">
        <el-menu-item index="2">添加设备</el-menu-item>
      </router-link>
    </el-menu>
  </el-header>
  <el-main>
    <router-view />
  </el-main>
</template>

