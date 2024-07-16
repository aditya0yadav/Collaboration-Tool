import {Navigate} from 'react-router-dom';
import { jwtDecode } from 'jwt-decode';
import rustApi from '../service/rustapi';
import { REFRESH_TOKEN, ACCESS_TOKEN } from '../../constant';
import { useEffect } from 'react';

function ProtectedRoute({children}) {
    const [isAuthorized, setIsAuthorized] = useState(false);

    useEffect(() =>{
        auth.catch(() => setIsAuthorized(false));
    })

    const refreshToken = async() => {
        const refreshToken = localStorage.getItem(REFRESH_TOKEN);
        

        try {
            const response = await rustApi.post('/refresh', {
                refresh : refreshToken,
            });
            console.log(response)
            if (response.status === 200) {
                localStorage.setItem(ACCESS_TOKEN, res.data.access)
                setIsAuthorized(true)
            }else{
                setIsAuthorized(false)
            }

        } catch(error) {
            console.log(`pr id  ${error}`);
            setIsAuthorized(false)

        }
    };

    const auth = async() => {
        const token = localStorage.getItem(ACCESS_TOKEN);
        if (!token) {
            setIsAuthorized(false)
            return

        }
        const decoded = jwtDecode(token);
        const tokenExpiration = decoded.exp;

        const now = Date.now()/1000;

        if (tokenExpiration < now){
            await refreshToken()

        }else {
            setIsAuthorized(true)
    }
    


    };
    if (isAuthorized == null) {
        return <div>Loading...</div>
    }
    return isAuthorized ? children : <Navigate to="/login"/>

}

