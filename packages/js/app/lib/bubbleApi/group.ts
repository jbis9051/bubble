import { UserLocal } from './user';

export class GroupService {
    static async get_groups(): Promise<Group[]> {
        return [];
    }
    static async create_group(): Promise<uuid> {
        return '';
    }
    static async add_member(group_uuid: uuid, user: uuid): Promise<void> {}
    static async remove_member(group_uuid: uuid, user: uuid): Promise<void> {}
    static async leave_group(group_uuid: uuid): Promise<void> {}
}

type uuid = string;
interface Group {
    uuid: string;
    name: string;
    members: UserLocal[];
}
