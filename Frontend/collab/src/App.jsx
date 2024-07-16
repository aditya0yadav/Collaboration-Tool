import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import Navbar from './components/Navbar';
import Group from './page/Group';

function App() {
    return (
        <Router>
            <Navbar />
            <Routes>
                <Route path="/group" element={<Group />} />
            </Routes>
        </Router>
    );
}

export default App;
