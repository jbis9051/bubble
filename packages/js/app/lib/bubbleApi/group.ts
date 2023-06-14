import { useEffect, useState } from 'react';
import { UserLocal } from './user';
import { useDispatch } from 'react-redux';
import { setGroups } from '../../redux/slices/groupSlice';

export class GroupService {
    static async get_groups(): Promise<Group[]> {
        return [
            {
                "members": [{ "name": "Apple" }],
                "name": "Bubble One",
                "uuid": "uuid-1"
            },
            {
                "members": [{ "name": "Banana" }],
                "name": "Bubble Two",
                "uuid": "uuid-2"
            },
            {
                "members": [{ "name": "Orange" }, { "name": "Apple" }, { "name": "Banana" }, { "name": "Pear" }],
                "name": "Bubble Three",
                "uuid": "uuid-3"
            },
            {
                "members": [{ "name": "Mango" }],
                "name": "Bubble Bubble Bubble Bubble Bubble Bubble",
                "uuid": "uuid-4"
            },
        ];
    }
    static async create_group(): Promise<uuid> {
        return '';
    }
    static async add_member(group_uuid: uuid, user: uuid): Promise<void> { }
    static async remove_member(group_uuid: uuid, user: uuid): Promise<void> { }
    static async leave_group(group_uuid: uuid): Promise<void> { }
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
