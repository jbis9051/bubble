import { useEffect, useState } from 'react';
import { UserLocal } from './user';
import { useDispatch } from 'react-redux';
import { setGroups } from '../../redux/slices/groupSlice';

export class GroupService {
    static async get_groups(): Promise<Group[]> {
        return [
            {
                "members": [{ "name": "Apple Core", "user_uuid": "user-uuid-apple" }],
                "name": "Bubble One",
                "uuid": "uuid-1"
            },
            {
                "members": [{ "name": "Banana Peel", "user_uuid": "user-uuid-banana" }],
                "name": "Bubble Two",
                "uuid": "uuid-2"
            },
            {
                "members": [{ "name": "Orange Slice", "user_uuid": "user-uuid-orange" }, { "name": "Apple Core", "user_uuid": "user-uuid-apple" }, { "name": "Banana Peel", "user_uuid": "user-uuid-banana" }, { "name": "Pear Stem", "user_uuid": "user-uuid-pear" }],
                "name": "Bubble Three",
                "uuid": "uuid-3"
            },
            {
                "members": [{ "name": "Mango Cube", "user_uuid": "user-uuid-mango" }],
                "name": "Bubble Bubble Bubble Bubble Bubble Bubble",
                "uuid": "uuid-4"
            },
        ];
    }
    static async create_group(group_name: string): Promise<uuid> {
        return 'uuid-new';
    }
    static async add_member(group_uuid: uuid, user: uuid): Promise<void> { }
    static async remove_member(group_uuid: uuid, user: uuid): Promise<void> { }
    static async leave_group(group_uuid: uuid): Promise<void> { }

    static async invite_user(group_uuid: uuid, username: string): Promise<void> { }
}

export function useGroups() {
    const [loaded, setLoaded] = useState(false);
    const dispatch = useDispatch();

    useEffect(() => {
        GroupService.get_groups().then((groups) => {
            dispatch(setGroups(groups));
            setLoaded(true);
        });
    }, []);

    return { loaded };
}

export type uuid = string;
export interface Group {
    uuid: string;
    name: string;
    members: UserLocal[];
}
