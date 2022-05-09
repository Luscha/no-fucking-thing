<template>
    <ul class="btns-group" :class="classname">
        <li v-for="(btn, i) in btns" :key="i">
            <router-link v-if="!btn.path.startsWith('http')" :to="btn.path" class="btn" :class="btn.btnClass">
                {{ btn.title }}
                <span class="d-none d-xl-inline-block" v-if="istext">{{ btn.titleTwo }}</span>
            </router-link>
            <a v-else :href="btn.path" class="btn" :class="btn.btnClass">
                {{ btn.title }}
                <span class="d-none d-xl-inline-block" v-if="istext">{{ btn.titleTwo }}</span>
            </a>
        </li>
    </ul>
</template>

<script>
export default {
  name: 'ButtonGroup',
  props: ['btns', 'istext', 'classname']
}
</script>
