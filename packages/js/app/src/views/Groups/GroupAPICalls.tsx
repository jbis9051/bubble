import { Share } from 'react-native';

export const share = async (urlStr: string) => {
    try {
        const result = await Share.share({
            url: urlStr,
        });
        if (result.action === Share.sharedAction) {
            console.log('Shared');
        }
    } catch (error) {
        console.log(error);
    }
};

export const getAddPerson = async () => {
    try {
        /* const res = await fetch('idk', {
             method: 'GET',
         });
         const json = await res.json(); */
        const json = 'google.com';
        await share(json);
    } catch (error) {
        console.error(error);
    }
    return null;
};

export const getJoinGroup = async () => {
    try {
        /* const res = await fetch('idk', {
             method: 'GET',
         });
         const json = await res.json(); */
        const json = 'google.com';
        await share(json);
    } catch (error) {
        console.error(error);
    }
    return null;
};

export const getCreateGroup = async () => {
    try {
        /* const res = await fetch('idk', {
             method: 'GET',
         });
         const json = await res.json(); */
        const json = 'google.com';
        await share(json);
    } catch (error) {
        console.error(error);
    }
    return null;
};
