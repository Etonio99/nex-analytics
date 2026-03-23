import { useState } from 'react';
import { BiX } from 'react-icons/bi';
import { AppNotification } from '../types/app-notification';

interface NotificationListProps {
  notifications: AppNotification[];
  dismissingIds: Set<string>;
  dismiss: (id: string) => void;
}

const NotificationList = (props: NotificationListProps) => {
  const [doneAnimatingIn, setDoneAnimatingIn] = useState<Set<string>>(
    new Set()
  );

  function getAnimationClass(id: string) {
    if (props.dismissingIds.has(id)) return 'slide-out-animation';
    if (doneAnimatingIn.has(id)) return '';
    return 'slide-in-animation';
  }

  return (
    <div className="absolute top-0 right-0 z-100">
      {props.notifications.map((notification) => (
        <div
          key={notification.id}
          className={`cursor-pointer bg-sandstone-900 text-sandstone-50 p-4 m-2 rounded-md shadow shadow-sandstone-950/20 w-80 ${getAnimationClass(notification.id)}`}
          onClick={() => props.dismiss(notification.id)}
          onAnimationEnd={() => {
            if (!props.dismissingIds.has(notification.id)) {
              setDoneAnimatingIn((prev) => new Set(prev).add(notification.id));
            }
          }}
        >
          <div className="flex justify-between">
            <h3 className="text-sandstone-300 font-bold">
              {notification.title}
            </h3>
            <div className="text-sandstone-400 hover:bg-sandstone-700 rounded-md p-1 grid place-items-center">
              <BiX />
            </div>
          </div>
          <p className="text-sm">{notification.message}</p>
        </div>
      ))}
    </div>
  );
};

export default NotificationList;
