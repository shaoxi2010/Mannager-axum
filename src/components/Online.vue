<script setup>
import { onMounted, ref } from 'vue'
import axios from 'axios'
const tableData = ref([])

const deleteDevice = (uuid) => {
    console.log(`delete ${uuid}`)
    axios.delete("/devices", {
        header: { 
            'Content-Type': 'application/json'
        },
        data: {
            uuid: uuid
        }
    }).catch(function(reason) {
            alert("通讯异常")
        }
    )
}

const getDevices = () => {
    axios.get("/devices", { headers: { 'Content-Type': 'application/json' } })
        .then(function(resp) {
            console.log(resp)
            tableData.value = resp.data
        })
        .catch(function(reason) {
            console.log(reason)
            tableData.value = []
        })
}
onMounted(()=> {
    getDevices()
    setInterval(getDevices, 1000)
})
</script>

<template>
    <el-table :data="tableData" style="width: 100%">
        <el-table-column prop="uuid" label="UID" width="180" />
        <el-table-column prop="name" label="名称" width="180" />
        <el-table-column prop="ipaddr" label="IP地址" />
        <el-table-column prop="online" label="在线状态">
            <template #default="scope">
                <el-tag :type="scope.row.online ? 'success' : 'danger'" disable-transitions>{{ scope.row.online ? "Online" : "Offline" }}</el-tag>
            </template>
        </el-table-column>
        <el-table-column fixed="right" label="操作" width="120">
        <template #default="scope">
            <el-button link type="primary" size="small" @click.prevent="deleteDevice(scope.row.uuid)">
                删除设备
            </el-button>
        </template>
        </el-table-column>
    </el-table>
</template>