export function summarizeNames(names: string[]): string {
    const { length } = names;
    [...names].sort();

    switch (length) {
        case 0:
            return '';
        case 1:
            return names[0];
        case 2:
            return `${names[0]} and ${names[1]}`;
        default:
            return `${names[0]}, ${names[1]}, and ${length - 2} other${
                length > 3 ? 's' : ''
            }`;
    }
}

export function getInitials(name: string): string {
    let initials = '';
    const nameList = name.split(' '); // Split the name into an array of words

    for (const word of nameList) {
        initials += word[0].toUpperCase(); // Append the first letter of each word (capitalized)
    }

    return initials;
}
