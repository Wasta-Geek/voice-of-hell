import { useState } from 'react';
import ProfileManagement from '../components/profile/ProfileManagement';

function Profiles() {
  const [profileIndex, setProfileIndex] = useState<string | null>();

  return (
    <main className="container">
      <div>
        <ProfileManagement profileIndex={profileIndex} setProfileIndex={setProfileIndex}/>
      </div>
    </main>
  );
}

export default Profiles;