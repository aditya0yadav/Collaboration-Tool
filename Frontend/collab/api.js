// import axios from 'axios';
// import { ACCESS_TOKEN } from './constant';

// const pythonApiUrl = import.meta.env.VITE_PYTHON_API_URL;
// const rustApiUrl = import.meta.env.VITE_RUST_API_URL;

// const pythonApi = axios.create({
//     baseURL: pythonApiUrl,
// });

// const rustApi = axios.create({
//     baseURL: rustApiUrl,
// });

// pythonApi.interceptors.request.use(
//     (config) => {
//         const token = localStorage.getItem(ACCESS_TOKEN);
//         if (token) {
//             config.headers.Authorization = `Bearer ${token}`;
//         }
//         return config;
//     },
//     (error) => {
//         return Promise.reject(error);
//     }
// );


// rustApi.interceptors.request.use(
//     (config) => {
//         const token = localStorage.getItem(ACCESS_TOKEN);
//         if (token) {
//             config.headers.Authorization = `Bearer ${token}`;
//         }
//         return config;
//     },
//     (error) => {
//         return Promise.reject(error);
//     }
// );

// export { pythonApi, rustApi };
