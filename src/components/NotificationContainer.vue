<template>
  <div class="notification-container">
    <TransitionGroup name="notification" tag="div">
      <NotificationToast
        v-for="(notification, index) in notifications"
        :key="`${notification}-${index}`"
        :message="notification"
        @close="removeNotification(notification)"
      />
    </TransitionGroup>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useFeedbackStore } from '../stores/feedback'
import NotificationToast from './NotificationToast.vue'

const feedbackStore = useFeedbackStore()

const notifications = computed(() => feedbackStore.notifications)

function removeNotification(message: string) {
  feedbackStore.removeNotification(message)
}
</script>

<style scoped>
.notification-container {
  position: fixed;
  top: 1rem;
  right: 1rem;
  z-index: 1000;
  pointer-events: none;
}

.notification-container > * {
  pointer-events: auto;
  margin-bottom: 0.5rem;
}

.notification-enter-active,
.notification-leave-active {
  transition: all 0.3s ease;
}

.notification-enter-from {
  opacity: 0;
  transform: translateX(100%);
}

.notification-leave-to {
  opacity: 0;
  transform: translateX(100%);
}

.notification-move {
  transition: transform 0.3s ease;
}
</style>
