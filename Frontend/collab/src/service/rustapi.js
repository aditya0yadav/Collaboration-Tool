import axios from 'axios';
import { ACCESS_TOKEN } from '../../constant';

const rustApi = axios.create({
  baseURL: import.meta.env.VITE_RUST_API_URL,
});

rustApi.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem(ACCESS_TOKEN);
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  },
  (error) => {
    return Promise.reject(error);
  }
);

export default rustApi;
