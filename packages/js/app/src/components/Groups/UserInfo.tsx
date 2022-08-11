import React from 'react';
import { View, Text, TouchableWithoutFeedback } from 'react-native';
import { Region } from 'react-native-maps';
import ProfileImageTemplate from '../ProfileImageTemplate';

type UserLocation = {
    name?: string;
    location: Region;
};

const UserInfo: React.FunctionComponent<{
    user: UserLocation;
    setLocations: React.Dispatch<React.SetStateAction<Region[]>>;
    setGroups: React.Dispatch<React.SetStateAction<UserLocation[]>>;
}> = ({ user, setLocations, setGroups }) => (
    <TouchableWithoutFeedback
        onPress={() => {
            setLocations([user.location]);
            setGroups([user]);
        }}
    >
        <View
            style={{
                flexDirection: 'row',
                alignItems: 'center',
                margin: 10,
            }}
        >
            <ProfileImageTemplate source="" size={50} />
            <View
                style={{
                    flexDirection: 'column',
                    marginLeft: 10,
                }}
            >
                <Text style={{ fontSize: 24 }}>{user.name ?? 'Anonymous'}</Text>
                <Text>Last Updated: Now</Text>
                {user.location && (
                    <Text>
                        {user.location.longitude} {user.location.latitude}
                    </Text>
                )}
            </View>
        </View>
    </TouchableWithoutFeedback>
);

export default UserInfo;
