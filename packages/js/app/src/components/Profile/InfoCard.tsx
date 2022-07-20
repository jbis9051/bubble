import React from 'react';
import { View, Text } from 'react-native';
import { FontAwesomeIcon } from '@fortawesome/react-native-fontawesome';
import { IconProp } from '@fortawesome/fontawesome-svg-core';
import InfoCardTemplate from '../InfoCardTemplate';

import Colors from '../../constants/Colors';

interface InfoProps {
    title: string;
    detail: string;
    icon: IconProp;
}

const ChildrenComponent: React.FunctionComponent<InfoProps> = ({
    title,
    detail,
    icon,
}) => (
    <View>
        {icon ? (
            <View style={{ flexDirection: 'row' }}>
                <FontAwesomeIcon icon={icon} color={Colors.primary} />
                <Text style={{ marginLeft: 5 }}>{title}</Text>
            </View>
        ) : (
            <View>
                <Text>{title}</Text>
            </View>
        )}
        <Text
            style={{
                fontSize: 20,
                fontWeight: '500',
                marginTop: 10,
            }}
        >
            {detail}
        </Text>
    </View>
);

const InfoCard: React.FunctionComponent<InfoProps> = ({
    title = '',
    detail = '',
    icon,
}) => (
    <InfoCardTemplate>
        <ChildrenComponent title={title} detail={detail} icon={icon} />
    </InfoCardTemplate>
);

export default InfoCard;
