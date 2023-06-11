import { useEffect, useState } from 'react';
import { UserLocal } from './user';
import { useDispatch } from 'react-redux';

export class GroupService {
    static async get_groups(): Promise<Group[]> {
        return [
            {
                members: [{ name: "Apple" }],
                name: "Bubble One",
                uuid: "uuid-1"
            }
        ];
    }
    static async create_group(): Promise<uuid> {
        return '';
    }
    static async add_member(group_uuid: uuid, user: uuid): Promise<void> {}
    static async remove_member(group_uuid: uuid, user: uuid): Promise<void> {}
    static async leave_group(group_uuid: uuid): Promise<void> {}
}

export function useGroups() {
    const [loaded, setLoaded] = useState(false);
    const dispatch = useDispatch();

    useEffect(() => {
        
    }, []);

    return { loaded };
}

type uuid = string;
interface Group {
    uuid: string;
    name: string;
    members: UserLocal[];
}
