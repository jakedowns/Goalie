<template>
  <div>
    <div v-if="view === 'login'">
      <form @submit.prevent="login">
        <input type="email" v-model="email" placeholder="Email" required />
        <input type="password" v-model="password" placeholder="Password" required />
        <button type="submit">Login</button>
      </form>
      <a @click="view = 'register'">Register</a>
      <a @click="view = 'forgot'">Forgot Password</a>
    </div>

    <div v-else-if="view === 'register'">
      <form @submit.prevent="register">
        <input type="email" v-model="email" placeholder="Email" required />
        <input type="password" v-model="password" placeholder="Password" required />
        <input type="password" v-model="confirmPassword" placeholder="Confirm Password" required />
        <button type="submit">Register</button>
      </form>
      <a @click="view = 'login'">Login</a>
    </div>

    <div v-else-if="view === 'forgot'">
      <form @submit.prevent="forgotPassword">
        <input type="email" v-model="email" placeholder="Email" required />
        <button type="submit">Reset Password</button>
      </form>
      <a @click="view = 'login'">Login</a>
    </div>
  </div>
</template>

<script>
export default {
  data() {
    return {
      view: 'login',
      email: '',
      password: '',
      confirmPassword: '',
