<script setup lang="ts">
interface User {
  id: number
  name: string
  email: string
}

const { data: users } = await useFetch<User[]>('http://127.0.0.1:8080/users')
</script>

<template>
  <div>
    <h1>Users</h1>
    <Suspense>
      <DataTable :value="users" :table-class="['styled-table']">
        <Column field="id" header="ID"></Column>
        <Column field="name" header="Name"></Column>
        <Column field="email" header="Email"></Column>
      </DataTable>

      <template #fallback>
        <div class="loading-container">
          <div class="spinner"></div>
          <p>Loading users...</p>
        </div>
      </template>
    </Suspense>
  </div>
</template>

<style scoped>
.loading-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 3rem;
  gap: 1rem;
}

.styled-table {
  min-width: 50rem;
  border-radius: 8px;
}

:deep(.styled-table) {
  background-color: #f8f9fa !important;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1) !important;
}

:deep(.styled-table .p-datatable-thead > tr > th) {
  background: linear-gradient(90deg, #667eea 0%, #764ba2 100%) !important;
  color: white !important;
  font-weight: 600 !important;
}

:deep(.styled-table .p-datatable-tbody > tr) {
  border-bottom: 1px solid #e0e0e0 !important;
}

:deep(.styled-table .p-datatable-tbody > tr:hover) {
  background-color: grey;
}

.spinner {
  width: 40px;
  height: 40px;
  border: 4px solid #f3f3f3;
  border-top: 4px solid #667eea;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}

p {
  color: #666;
  font-size: 1rem;
}
</style>