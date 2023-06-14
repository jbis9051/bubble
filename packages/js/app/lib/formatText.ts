

export function summarizeNames(names: string[]): string {
    const length = names.length;
    names.sort();

    switch (length) {
        case 0:
            return "";
        case 1:
            return names[0];
        case 2:
            return `${names[0]} and ${names[1]}`;
        default:
            return `${names[0]}, ${names[1]}, and ${length - 2} other${length > 3 ? 's' : ''}`;
    }
}
