import React, { useState } from 'react';
import "../style/Navbar.css";
import logo from "../assets/logo.svg";
import arrow from "../assets/arrow.svg";
import { Link } from "react-router-dom";

function Navbar() {
    const [isDropdownOpen, setDropdownOpen] = useState(false);

    const toggleDropdown = () => {
        setDropdownOpen(!isDropdownOpen);
    };
    // document.getElementsByClassName("dropdown").style.color = "black";

    return (
        <div className="navbar">
            <div className="navbar-logo">
                <img src={logo} alt="Logo" height={50} width={320} />
            </div>
            <div className="navbar-links">
                <div className="navbar-item">
                    <Link to="/group" className="navbar-link">
                        <h2>Group</h2>
                        <img src={arrow} alt="Arrow" />
                    </Link>
                    <div className="dropdown purple">
                        <Link to="/subgroup1" className="dropdown-link">Subgroup 1</Link>
                        <Link to="/subgroup2" className="dropdown-link">Subgroup 2</Link>
                    </div>
                </div>
                <div className="navbar-item">
                    <Link to="/file" className="navbar-link">
                        <h2>File</h2>
                        <img src={arrow} alt="Arrow" />
                    </Link>
                    <div className="dropdown purple">
                        <Link to="/upload" className="dropdown-link">Upload</Link>
                        <Link to="/download" className="dropdown-link">Download</Link>
                    </div>
                </div>
                <div className="navbar-item">
                    <Link to="/announcement" className="navbar-link">
                        <h2>Announcement</h2>
                        <img src={arrow} alt="Arrow" />
                    </Link>
                    <div className="dropdown purple">
                        <Link to="/new" className="dropdown-link">New</Link>
                        <Link to="/archive" className="dropdown-link">Archive</Link>
                    </div>
                </div>
                <div className="navbar-item">
                    <Link to="/signin" className="navbar-link">
                        <h2>Sign In</h2>
                        {/* <img src={arrow} alt="Arrow" /> */}
                    </Link>
                </div>
                <div className="navbar-item">
                    <Link to="/about" className="navbar-link">
                        <h2>About</h2>
                        {/* <img src={arrow} alt="Arrow" /> */}
                    </Link>
                </div>
                <div className="navbar-item">
                    <Link to="/trending" className="navbar-link">
                        <h2>Trending</h2>
                        <img src={arrow} alt="Arrow" />
                    </Link>
                </div>
                <div className="navbar-item profile-menu" onMouseEnter={toggleDropdown} onMouseLeave={toggleDropdown}>
                    <h2>Profile</h2>
                    {isDropdownOpen && (
                        <div className="dropdown profile-dropdown">
                            <div className="dropdown-item settings">
                                <Link to="/settings" className="dropdown-link">Settings</Link>
                            </div>
                            <div className="dropdown-item edit-profile">
                                <Link to="/edit-profile" className="dropdown-link">Edit Profile</Link>
                            </div>
                            <div className="dropdown-item about-profile">
                                <Link to="/about-profile" className="dropdown-link">About</Link>
                            </div>
                            <div className="dropdown-item login">
                                <Link to="/login" className="dropdown-link">Login</Link>
                            </div>
                        </div>
                     )}
                </div>
            </div>
        </div>
    );
}

export default Navbar;